import psycopg2

# Conexión a PostgreSQL
host = "localhost"  # Cambia esto si tu base de datos está en otro servidor
database = "nexis"
user = "postgres"
password = "12345678"  # Asegúrate de que la contraseña sea correcta
port = 5432  # Puerto predeterminado de PostgreSQL

def conection_db_postgres():
    # Crear una conexión a la base de datos PostgreSQL
    try:
        connection = psycopg2.connect(
            host=host,
            database=database,
            user=user,
            password=password,
            port=port  # Especificar el puerto
        )
        print("Conexión exitosa a la base de datos PostgreSQL")
        return connection

    except Exception as e:
        print("Error al conectar a la base de datos:", e)
        return None 

def desconection_db_postgres(conection):
        #la conexión si están abiertos
        if conection:
            conection.close()
