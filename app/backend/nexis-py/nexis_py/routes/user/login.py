from flask import Blueprint, request, jsonify
from nexis_py.utils.auth.paseto import decrypt_paseto, extract_claims
from nexis_py.utils.auth.redis import get_redis_connection, get_session_data

auth_blueprint = Blueprint("auth", __name__)

@auth_blueprint.route("/login", methods=["POST"])
def login():
    # Extract cookie and handle session authentication
    session_uuid = request.cookies.get("session_uuid")
    if not session_uuid:
        return jsonify({"error": "Session UUID is missing"}), 401

    # Decrypt token and extract claims
    paseto_token = decrypt_paseto(session_uuid)
    claims = extract_claims(paseto_token)

    # Query Redis for session data
    redis_conn = get_redis_connection()
    session_data = get_session_data(redis_conn, claims["session_uuid"])
    
    if session_data:
        return jsonify(session_data), 200
    else:
        return jsonify({"error": "Session expired"}), 401