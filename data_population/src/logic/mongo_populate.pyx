from connection.mongo_connection import make_mongo_pool
from utils.dummy_data_utils import EMAIL_DOMAIN_NAMES, ADDRESS_STREET_SYNONYMS
from utils.aliases import ANIME_DIR
from logic.person import Person
from logic.location import Location
from logic.university import University
from utils.aliases import DATASETS

from datetime import datetime
import random
import pandas as pd
import os

cpdef generate_phone_number():
    cdef str prefix = "#?"
    cdef int part_one = random.randint(10,99)
    cdef int part_two = random.randint(10,99)
    cdef int part_three = random.randint(10000,99999)
    cdef str full = prefix + "-"+ str(part_one)+"-"+ str(part_two)+"-"+ str(part_three)
    
    return full

cpdef generate_people():
    for filename in os.listdir(ANIME_DIR):
        pass

cpdef generate_universities():
    cdef database_name = os.getenv("MONGO_INITDB_DATABASE")

    cdef pool = make_mongo_pool()
    cdef db = pool[database_name]
    cdef collection = db['universities']

    cdef df = pd.read_csv(DATASETS.get("universities"))
    df.reset_index(inplace=True)
    cdef data_dict = df.to_dict("records")
    print(f"{datetime.now().time()} - MongoDB: Flushing universities...")
    collection.drop()

    print(f"{datetime.now().time()} - MongoDB: Populating universities...")
    collection.insert_many(data_dict)
     print(f"{datetime.now().time()} - MongoDB: Done")
    #print(data_dict)



cpdef generate_locations():
    pass


