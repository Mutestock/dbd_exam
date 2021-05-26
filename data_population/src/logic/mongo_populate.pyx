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
    return  prefix + "-" + str(part_one) + "-" + str(part_one) + "-" + str(part_two)    


cpdef generate_email(df):
    cdef int random_index = random.randint(0, len(EMAIL_DOMAIN_NAMES) - 1)
    cdef str suffix = EMAIL_DOMAIN_NAMES[random_index]
    cdef str prefix = ''.join(df.sample().iloc[0])
    return f"{prefix}@{suffix}"


cpdef generate_road_name(df):
    cdef int random_index = random.randint(0, len(ADDRESS_STREET_SYNONYMS) - 1)
    cdef str street_synonym = ADDRESS_STREET_SYNONYMS[random_index]
    cdef int street_number = random.randint(1,999)
    cdef str street_name = ''.join(df.sample().iloc[0])
    return f"{street_name} {street_synonym} {street_number}"


cpdef grab_df_sample(df):
    cdef int random_index = random.randint(0, len(df) - 1)
    return df.iloc[random_index]


cpdef grab_prepared_random_df():
    return pd.read_csv(DATASETS.get("random_words")).astype("str")


cpdef grab_prepared_last_names_df():
    return (pd.read_csv(DATASETS.get('last_names'), delimiter=";")["surname"]).astype("str")


cpdef grab_prepared_first_names_df():
    return (pd.read_csv(DATASETS.get('first_names'))["name"]).astype("str")


cpdef grab_prepared_countries_df():
    return (pd.read_csv(DATASETS.get('countries'))["name"]).astype("str")


cpdef generate_people():
    cdef collection = get_mongo_collection("people")
    cdef int anime_dir_length = len(os.listdir(ANIME_DIR))
    cdef int count
    cdef str filename
    cdef time_start

    # Retrieving datasets to retrieve samples from.
    # This is for random data generation.
    cdef random_words_df = grab_prepared_random_df()
    cdef last_names_df = grab_prepared_last_names_df()
    cdef first_names_df = grab_prepared_first_names_df()

    print(f"{datetime.now().time()} - MongoDB: Flushing people...")
    collection.drop()

    time_start = time.time()
    print(f"{datetime.now().time()} - MongoDB: Populating people. {anime_dir_length} entries...")
    for count, filename in enumerate(os.listdir(ANIME_DIR)):
        if count % 2000 == 0:
            print(f"{datetime.now().time()} - MongoDB: {count} / {anime_dir_length} completed ...")

        collection.insert_one({
            "first_name": grab_df_sample(first_names_df),
            "last_name": grab_df_sample(last_names_df),
            "email": generate_email(random_words_df),
            "phone_number": generate_phone_number(),
            "avatar": filename
        })
    print(f"{datetime.now().time()} - MongoDB: Done in {time.time() - time_start} seconds")


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
    cdef int i
    cdef random_words_df = grab_prepared_random_df()
    cdef countries_df = grab_prepared_countries_df()
    cdef int iter_count = 40000
    cdef time_start

    print(f"{datetime.now().time()} - MongoDB: Flushing locations...")
    collection.drop()
    print(f"{datetime.now().time()} - MongoDB: Populating locations...")
    time_start = time.time()
    for i in range(iter_count):
        if i % 2000 == 0:
            print(f"{datetime.now().time()} - MongoDB: {i} / {iter_count} completed ...")
        collection.insert_one({
            "street_name": generate_road_name(random_words_df),
            "zipcode": str(random.randint(10000, 99999)),
            "country":grab_df_sample(countries_df).capitalize()
        })
    print(f"{datetime.now().time()} - MongoDB: Done in {time.time() - time_start} seconds")










