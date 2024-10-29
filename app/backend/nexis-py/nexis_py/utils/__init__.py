import redis
from pymongo.mongo_client import MongoClient
from pymongo.server_api import ServerApi
from nexis_py.settings import get_settings, Settings

def get_redis_conn():
    settings: Settings = get_settings()
    client = redis.from_url(settings.redis.uri)
    return client

def get_mongo_conn():
    settings: Settings = get_settings()
    client = MongoClient(settings.database.uri, server_api=ServerApi('1'))
    db = client[settings.database.name]
    return db