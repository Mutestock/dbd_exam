from connection.mongo_connection import make_mongo_pool
from utils.dummy_data_utils import EMAIL_DOMAIN_NAMES, ADDRESS_STREET_SYNONYMS
from utils.aliases import ANIME_DIR
from logic.person import Person
from logic.location import Location
from logic.university import University

import random
import pandas as pd
import os

cpdef generate_phone_number():
    cdef str prefix = "#?"
    cdef int part_one = random.randint(10,99)
    cdef int part_two = random.randint(10,99)
    cdef int part_three = random.randint(10000,99999)
    cdef str full = prefix + "-"+ str(part_one)+"-"+ str(part_two)+"-"+ str(part_three)
    cdef stuff = make_mongo_pool()
    
    return full

cpdef generate_people():
    for filename in os.listdir(ANIME_DIR):
        pass

cpdef generate_universities():
    pass

cpdef generate_locations():
    pass


