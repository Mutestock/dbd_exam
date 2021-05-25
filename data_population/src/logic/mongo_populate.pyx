from connection.mongo_connection import make_mongo_pool, get_mongo_collection
from utils.dummy_data_utils import EMAIL_DOMAIN_NAMES, ADDRESS_STREET_SYNONYMS
from utils.aliases import ANIME_DIR
from logic.person import Person
from logic.location import Location
from logic.university import University
from utils.aliases import DATASETS
from utils.dummy_data_utils import ADDRESS_STREET_SYNONYMS, EMAIL_DOMAIN_NAMES

from datetime import datetime
import random
import pandas as pd
import os
import time

cpdef generate_phone_number():
    cdef str prefix = "#?"
    cdef int part_one = random.randint(10,99)
    cdef int part_two = random.randint(100,999)
    cdef str full = prefix + "-" + str(part_one) + "-" + str(part_one) + "-" + str(part_two)
    
    return full

cpdef generate_email(df):
    cdef int random_index
    cdef str suffix
    cdef str prefix = ''.join(df.sample())
    
    random_index = random.randint(0, len(EMAIL_DOMAIN_NAMES) -1)
    suffix = EMAIL_DOMAIN_NAMES[random_index]
    return f"{prefix}@{suffix}"

cpdef grab_first_name(df):
    cdef str first_name = str(df.sample().iloc[0])
    return first_name


cpdef grab_last_name(df):
    cdef str last_name = str(df.sample().iloc[0])
    return last_name


cpdef generate_people():
    cdef collection = get_mongo_collection("people")
    cdef int anime_dir_length = len(os.listdir(ANIME_DIR))
    cdef int count
    cdef str filename
    cdef time_start

    # Retrieving datasets to retrieve samples from.
    # This is for random data generation.
    cdef email_df = pd.read_csv(DATASETS.get("random_words_for_email_generation"))
    cdef last_names_df = pd.read_csv(DATASETS.get('last_names'), delimiter=";")
    last_names_df = last_names_df["surname"]
    cdef first_names_df = pd.read_csv(DATASETS.get('first_names'))
    first_names_df = first_names_df["name"]


    print(f"{datetime.now().time()} - MongoDB: Flushing people...")
    collection.drop()

    time_start = time.time()
    print(f"{datetime.now().time()} - MongoDB: Populating people. {anime_dir_length} entries...")
    for count, filename in enumerate(os.listdir(ANIME_DIR)):
        if count % 1000 == 0:
            print(f"{datetime.now().time()} - MongoDB: {count} / {anime_dir_length} completed ...")
        collection.insert_one({
            "first_name": grab_first_name(first_names_df),
            "last_name": grab_last_name(last_names_df),
            "email": generate_email(email_df),
            "phone_number": generate_phone_number(),
            "avatar": filename
        })
    print(f"{datetime.now().time()} - MongoDB: Done in {time_start - time.time()} seconds")

cpdef generate_universities():
    cdef collection = get_mongo_collection("universities")

    cdef df = pd.read_csv(DATASETS.get("universities"))
    df.reset_index(inplace=True)
    cdef data_dict = df.to_dict("records")

    print(f"{datetime.now().time()} - MongoDB: Flushing universities...")

    collection.drop()

    print(f"{datetime.now().time()} - MongoDB: Populating universities...")

    collection.insert_many(data_dict)

    print(f"{datetime.now().time()} - MongoDB: Done")



cpdef generate_locations():
    cdef collection = get_mongo_collection("locations")







