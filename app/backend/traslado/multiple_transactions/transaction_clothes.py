import uuid

def transaction_clothe(cursor,cursor2):
    # Contador para verificar cuántos documentos se procesan
    count = 0

    for document in cursor:
        count += 1  # Incrementar el contador por cada documento procesado

        # Generar UUIDs
        clothes_uuid = uuid.uuid4()
        type_uuid = uuid.uuid4()  # Asumiendo que tienes un tipo de ropa correspondiente

        try:
            # Insertar en ClothesTypes si no existe
            cursor2.execute("""
                INSERT INTO ClothesTypes (uuid, typeName)
                VALUES (%s, %s)
                ON CONFLICT (uuid) DO NOTHING
            """, (type_uuid, document['type']))

            # Insertar en Clothes
            cursor2.execute("""
                INSERT INTO Clothes (uuid, typeClothesUuid, clothesName, clothesPrice, clothesBrand, clothesSize, clothesGender, clothesAge)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s)
            """, (
                clothes_uuid,
                type_uuid,
                document['name'],
                float(document['price']),  # Asegúrate de que el tipo sea correcto
                document['brand'],
                document['size'],
                document['gender'],
                document['age']
            ))

             # Insertar colores
            for color in document['colors']:  # Asegúrate de que el nombre del campo es 'colors'
                color_uuid = uuid.uuid4()  # Generar un UUID para cada color
                cursor2.execute("""
                    INSERT INTO Colors (uuid, colorName)
                    VALUES (%s, %s)
                    ON CONFLICT (uuid) DO NOTHING
                """, (color_uuid, color))

                # Insertar en ClothesColors
                cursor2.execute("""
                    INSERT INTO ClothesColors (clothesUuid, colorUuid)
                    VALUES (%s, %s)
                """, (clothes_uuid, color_uuid))

        except Exception as e:
            print(f"Error al insertar el documento {document}: {e}")

    print(f"Total de documentos procesados: {count}")
