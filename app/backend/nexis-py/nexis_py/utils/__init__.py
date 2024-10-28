import redis
from pymongo import MongoClient
from nexis_py.settings import get_settings, Settings

def get_redis_conn():
    settings: Settings = get_settings()
    client = redis.from_url(settings.redis.uri)
    return client

def get_mongo_conn():
    settings: Settings = get_settings()
    client = MongoClient(settings.database.uri)
    db = client[settings.database.name]
    return db