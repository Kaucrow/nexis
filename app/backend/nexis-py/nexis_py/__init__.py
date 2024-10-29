from fastapi import FastAPI
from . import routes
from .routes import users

def create_app():
    app = FastAPI()

    # Register routes
    app.include_router(routes.router)
    app.include_router(users.router, prefix="/users")

    return app