import cli
from logic import mongo_populate
from logic import meili_populate
from connection.mongo_connection import make_mongo_pool
from connection.meili_connection import make_meili_pool

from time import time
from datetime import datetime

if __name__ == "__main__":
    print(
        """
          Some of these processes will take some time
          There's a lot data which is randomly fetched from some csv files.
          Expect like 5~ minutes total.
        """
    )
    time_start = time()
    meili_populate.populate_meili_people()
    # pyximport.install()
    # thing = mongo_populate.generate_phone_number()
    #
    # pool = make_mongo_pool()
    # db = pool["softdbd_db"]
    # collection = db["universities"]
    # print("Document count = ", collection.count_documents({}))
    # print(thing)
    mongo_populate.generate_universities()
    mongo_populate.generate_locations()
    mongo_populate.generate_people()
    meili_populate.populate_meili_people()
    # cli.manager()
    
    print(
        f"{datetime.now().time()} - ヽ༼ຈل͜ຈ༽ﾉ All Done in {time() - time_start} seconds"
    )
    print("Exiting...")
