import redis
from nexis_py.settings import get_settings, Settings

def get_redis_conn():
    settings: Settings = get_settings()
    client = redis.from_url(settings.redis.uri)
    return client