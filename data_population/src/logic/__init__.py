import pyximport

print("Building Cython")
pyximport.install(inplace=True, build_dir=".", language_level=3)
import logic.mongo_populate
import logic.meili_populate


