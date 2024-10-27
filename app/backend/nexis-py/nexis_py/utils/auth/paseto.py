from pyseto import Paseto
from nexis_py.settings import Settings

def decrypt_paseto(token):
    paseto = Paseto()
    return paseto.decrypt(token, Settings.PASETO_SECRET)

def extract_claims(token):
    # Example function to extract claims from a decrypted token
    return {"session_uuid": token["session_uuid"], "user_id": token.get("user_id")}
