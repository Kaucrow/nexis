from flask import Flask
from .routes import root_blueprint
from .routes.users import users_blueprint

def create_app():
    app = Flask(__name__)

    # Register blueprints
    app.register_blueprint(root_blueprint)
    app.register_blueprint(users_blueprint, url_prefix="/users")

    return app