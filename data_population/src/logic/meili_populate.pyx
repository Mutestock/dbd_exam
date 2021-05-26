from connection.meili_connection import make_meili_pool
from connection.mongo_connection import get_mongo_collection

from datetime import datetime
from time import time
from pprint import pprint
import json
import math

cpdef populate_meili_people():
    cdef meili_pool = make_meili_pool()
    cdef mongo_collection = get_mongo_collection("people")
    cdef count_column_entries = mongo_collection.count_documents({})
    cdef all_people
    cdef int chunk_size = 1000
    cdef int i
    cdef int j
    cdef time_start = time()

    print(f"{datetime.now().time()} - MongoDB: Amount of entries: {count_column_entries}...")
    print(f"{datetime.now().time()} - MongoDB: Grabbing all rows...")
    all_people = list(mongo_collection.find())

    print(f"{datetime.now().time()} - Formatting for Meili...")
    all_people = [{k:v for k,v in entry.items() if k!= "_id"} for entry in all_people]
    all_people = [{"person_id" if k == "row_number" else k:v for k,v in entry.items()} for entry in all_people]

    _index_resetter(meili_pool, "people_id", "people")

    print(f"{datetime.now().time()} - Meilisearch: Populating...")
    _populater(meili_pool, chunk_size, "people", all_people)

    print(f"{datetime.now().time()} - Done in {time() - time_start} seconds")

cpdef _index_resetter(meili_pool, str id_name, str index_name):
    try:
        print(f"{datetime.now().time()} - Meilisearch: Resetting {index_name} index...")
        meili_pool.index(index_name).delete()
        meili_pool.create_index(index_name, {
            "primaryKey": id_name,
            "name": index_name
        })
    except: 
        print(f"{datetime.now().time()} - Meilisearch: Create index from columns...")
        meili_pool.create_index(index_name, {
            "primaryKey": id_name,
            "name": index_name
        })
    finally:
        meili_pool.index('people').delete_all_documents()

cpdef _populater(meili_pool, int chunk_size, str index_name, content):
    cdef int updated_range
    cdef int indexer = math.floor(len(content)/chunk_size)
    cdef int extra = len(content)%chunk_size

    print(f"{datetime.now().time()} - Meilisearch: Populating...")

    for i in range(0, indexer):
        updated_range = i*chunk_size
        if updated_range % 5000 == 0:
            print(f"{datetime.now().time()} - Meilisearch: {updated_range} / {len(content)} completed ...")        
        list_thing = []
        if i==indexer:
            for j in range(updated_range, extra):
                list_thing.append(content[j])
        else:
            for j in range(updated_range, (i+1)*(chunk_size)):
                list_thing.append(content[j])
        meili_pool.index(index_name).add_documents(list_thing)

cpdef populate_meili_university():
    pass

cpdef populate_meili_locations():
    pass