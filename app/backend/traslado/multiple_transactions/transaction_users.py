import uuid
import os

def transaction_user(cursor, cursor2):
    # Contador para verificar cuántos documentos se procesan
    count = 0
    
    for document in cursor:
        count += 1  # Incrementar el contador por cada documento procesado

        # Generar UUIDs
        user_uuid = uuid.uuid4()
       
        try:
            # Asegurarse de que isActive tenga un valor válido
            is_active = document.get('isActive', False)  # Default to False if not present

            cursor2.execute("""
                INSERT INTO Users (uuid, nameUser, username, email, isActive, password)
                VALUES (%s, %s, %s, %s, %s, %s)
                ON CONFLICT (uuid) DO NOTHING
            """, (
                user_uuid,
                document['name'],  
                document['username'],
                document['email'],
                is_active,  # Asegurarse de que isActive tenga un valor
                document['password']
            ))

            # Insertar cliente si existe en el documento
            if 'client' in document and document['client'] is not None:
                client = document['client']
                client_uuid = uuid.uuid4()  # Generar un UUID para el cliente
                cursor2.execute("""
                    INSERT INTO Clients (clientUuid, phone, age, gender)
                    VALUES (%s, %s, %s, %s)
                """, (
                    user_uuid,  
                    client.get('phoneNum', None),  # Acceso seguro a phoneNum
                    client.get('age', None),
                    client.get('gender', None)
                ))

            # Insertar empleado si existe en el documento
            if 'employee' in document and document['employee'] is not None:
                employee = document['employee']
                employee_uuid = uuid.uuid4()  # Generar un UUID para el empleado
                cursor2.execute("""
                    INSERT INTO Employees (employeeUuid, phone, age, gender)
                    VALUES (%s, %s, %s, %s)
                """, (
                    user_uuid,
                    employee.get('phoneNum', None),  # Acceso seguro a phoneNum
                    int(employee.get('age', 0)),  # Default to 0 if age is missing
                    employee.get('gender', None)
                ))

        except Exception as e:
            print(f"Error al insertar el documento {document}: {e}")

    print(f"Total de documentos procesados de usuarios: {count}")