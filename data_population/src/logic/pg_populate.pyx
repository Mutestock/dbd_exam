from connection.pg_connection import engine
from connection.mongo_connection import get_mongo_collection

from sqlalchemy import Column, Integer, String, ForeignKey, Table, DateTime
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
from sqlalchemy.sql import func


base = declarative_base()

cdef class Location(Base):
    __tablename__ = "locations"

    id = Column(Integer, primary_key=True)
    street_name = Column(String(500), nullable=False)
    zipcode = Column(String(50), nullable=False)
    country = Column(String(500), nullable=False)


cdef class University(Base):
    __tablename__ = "universities"

    id = Column(Integer, primary_key=True)
    country_index = Column(String(50), nullable=False)
    university_name = Column(String(500), nullable=False)
    website_url = Column(String(500), nullable=False)

    location_id = Column(Integer, ForeignKey("location.id"))
    location = relationship("Location", backref=backref("universities", uselist=False))

cdef class Person(Base):
    __tablename__ = "people"

    id = Column(Integer, primary_key=True)
    first_name = Column(String(100), nullable=False)
    last_name = Column(String(100), nullable=False)
    email = Column(String(500), nullable=False)
    phone_number = Column(String(50), nullable=False)
    avatar = Column(String(100), nullable=False)
    created_at = Column(DateTime(timezone=True)), server_default=func.now())
    updated_at = Column(DateTime(timezone=True)), onupdate=func.now())

    university_id = Column(Integer, ForeignKey("universities.id"))
    university = relationship("University")

    university_id = Column(Integer, ForeignKey("locations.id"))
    university = relationship("Location")

cpdef reset_postgres():
    try:
        Person.__table__.drop()
    except:
        Person.__table__.drop(engine)
    try:
        University.__table__.drop()
    except:
        University.__table__.drop(engine)
    try:
        Location.__table__.drop()
    except:
        Location.__table__.drop(engine)

    Base.metadata.create_all(engine)


cpdef mass_populate_postgres():
    pass

    

