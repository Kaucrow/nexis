
# Crear esquema en PostgreSQL si aún no lo ha hecho
# Ruta al archivo .sql
sql_file_path = "db-model/postgres/NexisTables.sql"
def create_squema(conexion):
    cursor = conexion.cursor()
    # Verificar si el esquema 'nexis' ya existe
    cursor.execute("SELECT schema_name FROM information_schema.schemata WHERE schema_name = 'nexis';")
    schemas = cursor.fetchall()

    if not schemas:
        # Leer el archivo y ejecutar las consultas
        try:
            with open(sql_file_path, 'r') as file:
                sql_script = file.read()  # Leer el contenido del archivo

            # Ejecutar el script SQL
            cursor.execute(sql_script)
            conexion.commit()  # Confirmar los cambios
            print("Esquema creado exitosamente")

        except Exception as e:
            print("Error al ejecutar el script SQL:", e)
            if conexion:
                conexion.rollback()  # Revertir cambios en caso de error
    else:
        print("El esquema 'nexis' ya existe en la base de datos")