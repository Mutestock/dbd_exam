
from logic import mongo_populate
from logic import meili_populate
from logic import pg_populate
import utils.environment


from time import time
from datetime import datetime

if __name__ == "__main__":
    print(
        """
          Some of these processes will take some time.
          Much of the data is randomly generated.
          Expect like 3~ minutes total.
        """
    )
    time_start = time()
    mongo_populate.generate_universities()
    meili_populate.populate_meili_university()
    mongo_populate.generate_locations()
    meili_populate.populate_meili_locations()
    mongo_populate.generate_people()
    meili_populate.populate_meili_people()
    pg_populate.reset_tables()
    pg_populate.mass_populate()
    
    print(
        f"{datetime.now().time()} - ヽ༼ຈل͜ຈ༽ﾉ All Done in {time() - time_start} seconds"
    )
    print("Exiting...")
