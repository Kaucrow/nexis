from flask import jsonify
from . import root_blueprint

@root_blueprint.route("/health-check", methods=["GET"])
def health_check():
    return jsonify(
        "Python API is safe and healthy! :)"
    ), 200