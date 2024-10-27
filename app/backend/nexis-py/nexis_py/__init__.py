from flask import Flask
from nexis_py.settings import get_settings, Settings
from nexis_py.routes.user import auth_blueprint
from nexis_py.utils import red

def create_app():
    app = Flask(__name__)

    # Register blueprints
    app.register_blueprint(auth_blueprint, url_prefix="/auth")

    return app