cdef class University():
    def __cinit__(self, str country_initials, str name, str website_url):
        self.country_initials = country_initials
        self.name = name
        self.website_url = website_url