import cli
from logic import mongo_populate

if __name__ == '__main__':
    #pyximport.install()
    thing = mongo_populate.generate_phone_number()
    print(thing)
    #cli.manager()
    