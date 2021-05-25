from pymongo import MongoClient
import os

# utils.environment swaps environment variables on import
import utils.environment

MONGO_HOST = os.getenv("MONGO_HOST")
MONGO_PORT = os.getenv("MONGO_PORT")
MONGO_USERNAME = os.getenv("MONGO_INITDB_ROOT_USERNAME")
MONGO_PASSWORD = os.getenv("MONGO_INITDB_ROOT_PASSWORD")
MONGO_DATABASE = os.getenv("MONGO_INITDB_DATABASE")


def make_mongo_pool():
    #conn_str = f"mongodb://{MONGO_USERNAME}:{MONGO_PASSWORD}@{MONGO_HOST}:{MONGO_PORT}/{MONGO_DATABASE}?authSource=admin"
    conn_str = "localhost:31293"
    #client = MongoClient(
    #    host="localhost",
    #    username="softdbd_user",
    #    password="softdbd_pass",
    #    port=31293,
    #    authSource="admin",    
    #)
    #return client

    return MongoClient(conn_str)
    # print(conn_str)
    # return MongoClient(
    #    host=MONGO_HOST,
    #    port=int(MONGO_PORT),
    #    username=MONGO_USERNAME,
    #    password=MONGO_PASSWORD,
    #    authSource=MONGO_DATABASE
    # )
