from fastapi import Cookie
from nexis_py.utils.auth.tokens import verify_session_token
from nexis_py.types.responses import ErrorResponse
from . import router

@router.get("/verify-session")
def verify_session(sss_uuid_token: str = Cookie(None, alias="session_uuid")):
    if not sss_uuid_token:
        return ErrorResponse("`session_uuid` cookie is not present.", 400)

    response = verify_session_token(sss_uuid_token)
    return response