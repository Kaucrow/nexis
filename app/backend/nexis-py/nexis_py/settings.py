import os
import toml
from .utils import red

class DatabaseSettings:
    uri: str
    name: str

    def __init__(self, database_data):
        self.uri = database_data["uri"]
        self.name = database_data["name"]

class RedisSettings:
    uri: str

    def __init__(self, redis_data):
        self.uri = redis_data["uri"]

class ApplicationSettings:
    protocol: str
    host: str
    base_url: str
    port: int

    def __init__(self, application_data):
        self.protocol = application_data["protocol"]
        self.host = application_data["host"]
        self.base_url = application_data["base_url"]
        self.port = application_data["port"]

class SecretSettings:
    token_key: str
    hmac_secret: str

    def __init__(self, secret_data):
        self.token_key = secret_data["token_key"]
        self.hmac_secret = secret_data["hmac_secret"]

class Settings:
    database: DatabaseSettings
    redis: RedisSettings
    application: ApplicationSettings
    secret: SecretSettings
    debug: bool
    frontend_url: str

    def __init__(self, base_data, app_data):
        self.database = DatabaseSettings(base_data["database"])
        self.redis = RedisSettings(base_data["redis"])
        self.application = ApplicationSettings(app_data["application"])
        self.secret = SecretSettings(app_data["secret"])
        self.debug = app_data["debug"]
        self.frontend_url = app_data["frontend_url"]

def get_settings():
    app_env = os.getenv("APP_ENVIRONMENT", "development").lower()

    base_file = "base.toml"

    app_file = (
        "development.toml" if app_env == "development"
        else "production.toml" if app_env == "production"
        else None
    )

    if app_file is None:
        print(red("[ ERR ] ") + f"Unknown value found on APP_ENVIRONMENT: `{app_env}`. Please set it to either `development` or `production`.")
        exit(1)

    with open("settings/" + app_file, "r") as file:
        app_data = toml.load(file)
    
    with open("settings/" + base_file, "r") as file:
        base_data = toml.load(file)

    return Settings(base_data, app_data)