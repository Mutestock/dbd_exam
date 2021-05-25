import pyximport

pyximport.install(inplace=True, build_dir=".", language_level=3)
import logic.mongo_populate

stuff = mongo_populate.generate_phone_number()
print(stuff)

mongo_populate.generate_people()