from pymongo.mongo_client import MongoClient
from pymongo.server_api import ServerApi


# Definir el nombre de usuario y la contraseña como cadenas
username = "geanfranco"
password = "3A2V3NgNwC24myWq"

# URI de conexión a MongoDB
uri = f"mongodb+srv://{username}:{password}@primary.v0b5o.mongodb.net/?retryWrites=true&w=majority&appName=primary"

# Crear un nuevo cliente y conectar al servidor MongoDB
client = MongoClient(uri, server_api=ServerApi('1'))

def conetion_bd_mongo():
    # Enviar un ping para confirmar una conexión exitosa a MongoDB
    try:
        client.admin.command('ping')
        print("Conexión exitosa a la base de datos MongoDB!")
        return client 
    except Exception as e:
        print("Error al conectar a MongoDB:", e)
        return None 



