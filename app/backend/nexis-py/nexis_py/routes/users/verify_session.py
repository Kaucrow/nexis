from flask import request
from nexis_py.utils.auth.tokens import verify_session_token
from . import users_blueprint

@users_blueprint.route("/verify-session", methods=["GET"])
def verify_session():
    response = verify_session_token(request)
    return response