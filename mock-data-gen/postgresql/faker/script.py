import psycopg2
from faker import Faker

fake = Faker()

db_params = {
    'dbname': 'testingfaker', # Example Database
    'user': 'postgres', 
    'password': '', # Change this
    'host': 'localhost', #Default
    'port': '5432' # Default
}

def generateFakeData(num_entries = 100):
    data = []
    for _ in range(num_entries):
        entry = (
            fake.name(),
            fake.email(),
            fake.basic_phone_number(),
            fake.country(),
            fake.job()
        )
        data.append(entry)
    return data

def insertDataToDB(data):
    try:
        conn = psycopg2.connect(**db_params)
        cursor = conn.cursor()
        
        insert_query = """
        INSERT INTO fake_data (name, email, phone_number, location, profession)
        VALUES (%s, %s, %s, %s, %s);
        """
        cursor.executemany(insert_query, data)
        
        # Javier needed a commit to keep living
        conn.commit()
        
        print(f"{cursor.rowcount} records inserted successfully.")
        
    except Exception as e:
        print(f"Error: {e}")
    
    finally:
        cursor.close()
        conn.close()
    
fakeData = generateFakeData(100)
insertDataToDB(fakeData)


# Table Used 
#   CREATE TABLE fake_data (
#       id SERIAL PRIMARY KEY,
#       name VARCHAR(100),
#       email VARCHAR(100),
#       phone_number VARCHAR(20),
#       location VARCHAR(100),
#       profession VARCHAR(100)
#   );

#   
