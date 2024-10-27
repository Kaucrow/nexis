import json
from pyseto import pyseto, Key
from nexis_py.settings import Settings, get_settings

def decode_paseto(token):
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

def extract_claims(token):
    # Example function to extract claims from a decrypted token
    return {"session_uuid": token["session_uuid"], "user_id": token.get("user_id")}
