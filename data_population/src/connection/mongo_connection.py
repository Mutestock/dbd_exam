from pymongo import MongoClient
import os

# utils.environment swaps environment variables on import
import utils.environment

MONGO_HOST = os.getenv("MONGO_HOST")
MONGO_PORT = os.getenv("MONGO_PORT")
MONGO_USERNAME = os.getenv("MONGO_INITDB_ROOT_USERNAME")
MONGO_PASSWORD = os.getenv("MONGO_INITDB_ROOT_PASSWORD")
MONGO_DATABASE = os.getenv("MONGO_INITDB_DATABASE")


# I acknowledge the security risks of running the database without authentication
# MongoDB's authentication can be strict, and I've been fighting with it.
# Not enough time to create the correct solution

def make_mongo_pool():
    conn_str = "localhost:31293"
    return MongoClient(conn_str)

def get_mongo_collection(collection):
    pool = make_mongo_pool()
    db = pool[MONGO_DATABASE]
    return db[collection]
