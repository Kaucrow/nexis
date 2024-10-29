import json
from pyseto import pyseto, Key
from bson.objectid import ObjectId
from nexis_py.settings import Settings, get_settings
from nexis_py.types.responses import ErrorResponse
from nexis_py.utils import get_mongo_conn, get_redis_conn

SESSION_KEY_PREFIX = "session_"

# Decodes a PASETO token and returns its claims.
# Raises an exception if the decoding fails.
def get_token_claims(token):
    settings: Settings = get_settings()

    key = Key.new(version=4, purpose="local", key=settings.secret.token_key.encode())
    hmac_secret = settings.secret.hmac_secret.encode()

    decoded = pyseto.decode(
        key,
        token=token,
        implicit_assertion=hmac_secret
    )

    claims = json.loads(decoded.payload.decode())

    return claims

# Builds a PASETO token with a set of claims provided,
# as a dictionary, and returns the token as bytes.
def build_token(claims):
    settings: Settings = get_settings()

    key = Key.new(version=4, purpose="local", key=settings.secret.token_key.encode())
    hmac_secret = settings.secret.hmac_secret.encode()

    token = pyseto.encode(
        key,
        payload=claims,
        implicit_assertion=hmac_secret
    )

    return token

# Verifies a session based on the session UUID token
# from the cookies, and the session data token from redis.
# Returns a tuple where the first element is a message, and
# the second is an HTTP response code.
def verify_session_token(request):
    # Extract cookie
    session_uuid_token = request.cookies.get("session_uuid")
    if not session_uuid_token:
        return ErrorResponse("Session cookie is missing.", 400)

    try:
        claims = get_token_claims(session_uuid_token)

        try:
            sss_uuid = claims["session_uuid"]
        except Exception:
            return ErrorResponse("`session_uuid` claim is missing.", 400)

        try:
            redis_conn = get_redis_conn()
        except Exception:
            return ErrorResponse("Failed to establish redis connection.", 500)

        # Get the session data token from redis
        sss_token = redis_conn.get(f"{SESSION_KEY_PREFIX}{sss_uuid}")

        if sss_token:
            settings: Settings = get_settings()

            # Reset the session expiration
            redis_conn.expire(f"{SESSION_KEY_PREFIX}{sss_uuid}", settings.secret.session_token_expiration * 60)

        # If the session expired, try to renew it.
        # This happens when the session UUID token exists but the session UUID
        # doesn't match any key in redis.
        else:
            db = get_mongo_conn()

            # Try to get the `user_id` from the session uuid token claims.
            # If it doesn't exist, it means that "Remember me" was not set,
            # and the session can't be renewed.
            try:
                user_id = claims["user_id"]["$oid"]
            except Exception:
                return ErrorResponse("Session expired and `user_id` claim is not present.", 401)
            
            users_coll = db['user']

            # Get the user object from the database
            user = users_coll.find_one({
                "_id": ObjectId(user_id)
            })

            # Renew the session in redis
            sss_data_token = build_token({ "user_id": str(user["_id"]) })
            settings: Settings = get_settings()
            redis_conn.set(f"{SESSION_KEY_PREFIX}{sss_uuid}", sss_data_token, ex=settings.secret.session_token_expiration * 60)

        return "", 200 

    except Exception:
        return ErrorResponse("Failed to verify session.", 401)