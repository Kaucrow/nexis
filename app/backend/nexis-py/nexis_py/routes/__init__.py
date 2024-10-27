from flask import Blueprint

root_blueprint = Blueprint("/", __name__)

from . import health_check