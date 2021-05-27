from connection.pg_connection import make_pg_pool
from connection.mongo_connection import get_mongo_collection


from datetime import datetime
import time
import random

cpdef reset_tables():
    cdef conn = make_pg_pool()
    cdef cursor = conn.cursor()

    print(f"{datetime.now().time()} - PostgreSQL: Dropping tables...")
    cursor.execute("DROP TABLE IF EXISTS people")
    cursor.execute("DROP TABLE IF EXISTS universities")
    cursor.execute("DROP TABLE IF EXISTS locations")

    print(f"{datetime.now().time()} - PostgreSQL: Creating tables...")

    _create_locations_table(cursor)
    _create_universities_table(cursor)
    _create_people_table(cursor)

    conn.commit()
    cursor.close()
    conn.close()

# Exactly the same as migrations in backend

cdef _create_locations_table(cursor):
    cdef query = """
    CREATE TABLE IF NOT EXISTS locations(
        id SERIAL PRIMARY KEY NOT NULL,
        street_name VARCHAR(500) NOT NULL,
        zipcode VARCHAR(50) NOT NULL,
        country VARCHAR(500) NOT NULL
    );"""
    cursor.execute(query)

cdef _create_universities_table(cursor):
    cdef query = """
    CREATE TABLE IF NOT EXISTS universities(
        id SERIAL PRIMARY KEY NOT NULL,
        country_index VARCHAR(50) NOT NULL,
        university_name VARCHAR(500) NOT NULL,
        website_url VARCHAR(500) NOT NULL,
        locations_id INTEGER REFERENCES locations
    );"""
    cursor.execute(query)

cdef _create_people_table(cursor):
    cdef query = """
    CREATE TABLE IF NOT EXISTS people (
        id SERIAL PRIMARY KEY NOT NULL,
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        email VARCHAR(500) NOT NULL,
        phone_number VARCHAR(50) NOT NULL,
        avatar VARCHAR(100) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        university_id INTEGER REFERENCES universities,
        locations_id INTEGER REFERENCES locations
    );"""
    cursor.execute(query)

# It doesn't make sense to execute in any other way,
# So it's reduced to one function instead
cpdef mass_populate():
    _populate_locations()
    _populate_universities()
    _populate_people()

cdef _populate_locations():
    cdef conn = make_pg_pool()
    cdef cursor = conn.cursor()
    print(f"{datetime.now().time()} - MongoDB: Grabbing all locations...")
    cdef all_locations = list(get_mongo_collection("locations").find())
    cdef int collection_size = len(all_locations)
    cdef int i
    cdef str street_name
    cdef str zipcode
    cdef str country
    

    print(f"{datetime.now().time()} - PostgreSQL: Populating locations...")
    for i, location in enumerate(all_locations):
        if i % 2000 == 0:
            print(f"{datetime.now().time()} - PostgreSQL: {i} / {collection_size} completed ...")

        street_name = location.get('street_name')
        zipcode = location.get('zipcode')
        country = location.get('country')

        if "'" in street_name:
            street_name = street_name.replace("'","’\\’")

        if "'" in country:
            country = country.replace("'","\\'")

        cursor.execute(f"INSERT INTO locations VALUES(DEFAULT, E'{street_name}','{zipcode}', E'{country}')")

    conn.commit()
    cursor.close()
    conn.close()

cdef _populate_universities():
    cdef conn = make_pg_pool()
    cdef cursor = conn.cursor()
    print(f"{datetime.now().time()} - MongoDB: Grabbing all universities...")
    cdef all_universities = list(get_mongo_collection("universities").find())
    cdef int all_locations_size = get_mongo_collection("locations").count_documents({})
    cdef int collection_size = len(all_universities)
    cdef int i
    cdef str country_index
    cdef str university_name
    cdef str website_url
    cdef int locations_id

    print(f"{datetime.now().time()} - PostgreSQL: Populating universities...")
    for i, university in enumerate(all_universities):
        if i % 2000 == 0:
            print(f"{datetime.now().time()} - PostgreSQL: {i} / {collection_size} completed ...")

        country_index = university.get("country_initials")
        university_name = university.get("university_name")
        website_url = university.get("website_url")
        locations_id = random.randint(1, all_locations_size)

        if "'" in country_index:
            country_index = country_index.replace("'","’\\’")
        if "'" in university_name:
            university_name = university_name.replace("'","’\\’")
        if "'" in website_url:
            website_url = website_url.replace("'","’\\’")
        
        cursor.execute(f"INSERT INTO universities VALUES(DEFAULT, E'{country_index}', E'{university_name}', E'{website_url}', {locations_id})")        
        
cdef _populate_people():
    cdef conn = make_pg_pool()
    cdef cursor = conn.cursor()
    print(f"{datetime.now().time()} - MongoDB: Grabbing all people...")
    cdef all_people = list(get_mongo_collection("people").find())
    cdef int all_locations_size = get_mongo_collection("locations").count_documents({})
    cdef int all_universities_size = get_mongo_collection("universities").count_documents({})
    cdef int collection_size = len(all_people)
    cdef int i
    cdef str first_name
    cdef str last_name
    cdef str email
    cdef str phone_number
    cdef str avatar
    cdef int locations_id
    cdef int university_id

    print(f"{datetime.now().time()} - PostgreSQL: Populating people...")
    for i, person in enumerate(all_people):
        if i % 2000 == 0:
            print(f"{datetime.now().time()} - PostgreSQL: {i} / {collection_size} completed ...")

        first_name = person.get("first_name")
        last_name = person.get("last_name")
        email = person.get("email")
        phone_number = person.get("phone_number")
        avatar = person.get("avatar")
        locations_id = random.randint(1, all_locations_size)
        university_id = random.randint(1, all_universities_size)


        if "'" in first_name:
            first_name = first_name.replace("'","’\\’")
        if "'" in last_name:
            last_name = last_name.replace("'","’\\’")
        if "'" in email:
            email = email.replace("'","’\\’")
        if "'" in phone_number:
            phone_number = phone_number.replace("'","’\\’")
        if "'" in avatar:
            avatar = avatar.replace("'","’\\’")

        cursor.execute(f"INSERT INTO people VALUES(DEFAULT, E'{first_name}', E'{last_name}', E'{email}', E'{phone_number}', E'{avatar}', DEFAULT, DEFAULT, {university_id}, {locations_id})")        

