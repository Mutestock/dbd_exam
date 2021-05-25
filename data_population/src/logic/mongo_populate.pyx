import random
#from connection.mongo_connection import make_mongo_pool()

cpdef generate_phone_number():
    cdef str prefix = "#?"
    cdef int part_one = random.randint(10,99)
    cdef int part_two = random.randint(10,99)
    cdef int part_three = random.randint(10000,99999)
    cdef str full = prefix + "-"+ str(part_one)+"-"+ str(part_two)+"-"+ str(part_three)

    return full


