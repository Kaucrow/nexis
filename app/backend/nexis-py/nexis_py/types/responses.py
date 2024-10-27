from flask import jsonify, Response
from typing import Tuple

def ErrorResponse(msg: str, status=500) -> Tuple[Response, int]:
    response = {
        "error": msg
    }

    return jsonify(response), status