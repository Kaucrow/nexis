import redis
from nexis_py.settings import Settings

def get_redis_connection():
    return redis.from_url(Settings.REDIS_URL)

def get_session_data(redis_conn, session_uuid):
    return redis_conn.get(f"session:{session_uuid}")