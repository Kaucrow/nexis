from flask import request
from nexis_py.utils.auth.tokens import decode_paseto
from nexis_py.utils import get_redis_conn, get_mongo_conn
from nexis_py.types.responses import ErrorResponse
from bson.objectid import ObjectId
from . import users_blueprint

SESSION_KEY_PREFIX = "session_"

@users_blueprint.route("/verify-session", methods=["GET"])
def verify_session():
    # Extract cookie and handle session authentication
    session_uuid_token = request.cookies.get("session_uuid")
    if not session_uuid_token:
        return ErrorResponse("Session cookie is missing.", 400)

    try:
        claims = decode_paseto(session_uuid_token)

        try:
            sss_uuid = claims["session_uuid"]
        except Exception:
            return ErrorResponse("`session_uuid` claim is missing.", 400)

        try:
            redis_conn = get_redis_conn()
        except Exception:
            return ErrorResponse("Failed to establish redis connection.", 500)

        sss_token = redis_conn.get(f"{SESSION_KEY_PREFIX}{sss_uuid}")

        if sss_token is not None:
            print(sss_token)
            claims = decode_paseto(sss_token)
            print(claims["user_id"])
        else:
            print("REACHED RENEWAL")
            db = get_mongo_conn()

            try:
                user_id = claims["user_id"]
            except Exception:
                return ErrorResponse("Session expired and `user_id` claim is not present.", 401)
            
            users_coll = db['user']

            user = users_coll.find_one({
                "_id": ObjectId(user_id)
            })
        
            redis_conn.set(f"{SESSION_KEY_PREFIX}{sss_uuid}", )

        return "", 200
    except Exception:
        return ErrorResponse("Failed to verify session.", 401)