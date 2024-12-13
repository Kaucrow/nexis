import uuid

def transaction_techGpu(cursor, cursor2):
    # Contador para verificar cuántos documentos se procesan
    count = 0
    
    for document in cursor:
        count += 1  # Incrementar el contador por cada documento procesado

        # Generar UUIDs
        techCPUS_uuid = uuid.uuid4()
       
        try:
            cursor2.execute("""
                INSERT INTO TechGpu (uuid, gpuBrand, gpuModel, gpuPrice)
                VALUES (%s, %s, %s, %s)
                ON CONFLICT (uuid) DO NOTHING
            """, (
                techCPUS_uuid,
                document['brand'],  
                document['model'],
                document['price'],
            ))

        except Exception as e:
            print(f"Error al insertar el documento {document}: {e}")

    print(f"Total de documentos procesados de usuarios: {count}")