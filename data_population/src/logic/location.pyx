cdef class Location():
    def __cinit__(self, int postal_code, str street, str country):
        self.postal_code = postal_code
        self.street = street
        self.country = country