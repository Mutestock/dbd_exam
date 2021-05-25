cdef class Person():
    def __cinit__(self, str name, str phone_number, str email, str img_path):
        self.name = name
        self.phone_number = phone_number
        self.email = email
        self.img_path = img_path