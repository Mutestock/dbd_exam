from connection.meili_connection import make_meili_pool
from connection.mongo_connection import get_mongo_collection

from datetime import datetime
from time import time, sleep
from pprint import pprint
import json
import math

cpdef populate_meili_people():
    cdef meili_pool = make_meili_pool()
    cdef mongo_collection = get_mongo_collection("people")
    cdef count_column_entries = mongo_collection.count_documents({})
    cdef person_from_mongo
    cdef all_people
    cdef int chunk_size = 3000
    cdef int i
    cdef int j
    cdef time_start = time()

    print(f"{datetime.now().time()} - MongoDB: Grabbing all rows...")
    all_people = list(mongo_collection.find())

    print(f"{datetime.now().time()} - Formatting for Meili...")
    all_people = [{k:v for k,v in entry.items() if k!= "_id"} for entry in all_people]
    all_people = [{"person_id" if k == "row_number" else k:v for k,v in entry.items()} for entry in all_people]

    
    _index_resetter(meili_pool, "person_id", "people")
    _populater(meili_pool, chunk_size, "people", all_people)
    print(f"{datetime.now().time()} - Done in {time() - time_start} seconds")

cpdef populate_meili_university():
    cdef meili_pool = make_meili_pool()
    cdef mongo_collection = get_mongo_collection("universities")
    cdef str index_name = "universities"
    cdef str id_name = "index"
    cdef count_column_entries = mongo_collection.count_documents({})
    cdef all_universities
    cdef int chunk_size = 3000
    cdef time_start = time()

    print(f"{datetime.now().time()} - MongoDB: Amount of entries: {count_column_entries}...")
    print(f"{datetime.now().time()} - MongoDB: Grabbing all rows...")

    all_universities = list(mongo_collection.find())
    all_universities = _formatter(all_universities)
    
    _index_resetter(meili_pool, id_name, index_name)
    _populater(meili_pool, chunk_size, index_name, all_universities)
    print(f"{datetime.now().time()} - Done in {time() - time_start} seconds")


cpdef populate_meili_locations():
    cdef meili_pool = make_meili_pool()
    cdef mongo_collection = get_mongo_collection("locations")
    cdef str index_name = "locations"
    cdef str id_name = "location_id"
    cdef count_column_entries = mongo_collection.count_documents({})
    cdef all_locations
    cdef int chunk_size = 3000
    cdef time_start = time()

    print(f"{datetime.now().time()} - MongoDB: Amount of entries: {count_column_entries}...")
    print(f"{datetime.now().time()} - MongoDB: Grabbing all rows...")

    all_locations = list(mongo_collection.find())
    all_locations = _formatter(all_locations)
    
    cdef all_locations_formatted = []
    for i, location in enumerate(all_locations):
        location[id_name] = i
        all_locations_formatted.append(location)
    
    _index_resetter(meili_pool, id_name, index_name)
    _populater(meili_pool, chunk_size, index_name, all_locations_formatted)
    # meili_pool.index(index_name).add_documents([all_locations[0]])
    print(f"{datetime.now().time()} - Done in {time() - time_start} seconds")


cpdef _formatter(content):
    print(f"{datetime.now().time()} - Formatting for Meili...")
    return [{k:v for k,v in entry.items() if k!= "_id"} for entry in content]

cpdef _index_resetter(meili_pool, str id_name, str index_name):
    sleep(2)
    try:
        print(f"{datetime.now().time()} - Meilisearch: Create index {index_name}...")
        meili_pool.create_index(index_name, {
                "primaryKey": id_name,
                "name": index_name
        })
    except: 
        print(f"{datetime.now().time()} - Meilisearch: Resetting {index_name} index...")
        meili_pool.index(index_name).delete_all_documents()
        meili_pool.index(index_name).delete()
        meili_pool.create_index(index_name, {
                "primaryKey": id_name,
                "name": index_name
        })


cpdef _populater(meili_pool, int chunk_size, str index_name, content):
    cdef int updated_range
    cdef int indexer = math.ceil(len(content)/chunk_size)
    cdef int extra = len(content)%chunk_size
    cdef int i
    cdef int j

    print(f"{datetime.now().time()} - Meilisearch: Populating...")
    print(extra)

    for i in range(0, indexer):
        updated_range = i*chunk_size
        # v The database actually gets overloaded without this 
        sleep(0.2)
        if updated_range % 5000 == 0:
            print(f"{datetime.now().time()} - Meilisearch: {updated_range} / {len(content)} completed ...")        
        list_thing = []
        if i+1==indexer:
            for j in range(updated_range, updated_range+extra):
                list_thing.append(content[j])
        else:
            for j in range(updated_range, (i+1)*(chunk_size)):
                list_thing.append(content[j])
        meili_pool.index(index_name).add_documents(list_thing)

