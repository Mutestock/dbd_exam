import cli
from logic import mongo_populate
from connection.mongo_connection import make_mongo_pool

if __name__ == '__main__':
    #pyximport.install()
    thing = mongo_populate.generate_phone_number()
    mongo_populate.generate_universities()
    pool = make_mongo_pool()
    db = pool["softdbd_db"]
    collection = db["universities"]
    print("Document count = ", collection.find().count())
    print(thing)
    #mongo_populate.generate_locations()
    mongo_populate.generate_people()
    #cli.manager()
    