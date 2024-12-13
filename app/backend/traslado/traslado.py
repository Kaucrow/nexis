from conex_mong import *
from conex_postg import *
from create_squema import *
import psycopg2.extras
from multiple_transactions.transaction_clothes import *
from multiple_transactions.transaction_users import *
from multiple_transactions.transaction_tecnologi_CPU import *
from multiple_transactions.transaction_techGpu import *

# Registrar el adaptador para UUID
psycopg2.extras.register_uuid()

conexion1 = conetion_bd_mongo()
conexion2 = conection_db_postgres()
create_squema(conexion2)
cursor2 = conexion2.cursor()
mongo_db = conexion1['nexis']  # Conexión del cliente en la base de datos
 

mongo_collection = mongo_db['clothes']
cursor = mongo_collection.find()
transaction_clothe(cursor,cursor2)


# Confirmar la transacción
conexion2.commit()
cursor2.close()
conexion2.close()

