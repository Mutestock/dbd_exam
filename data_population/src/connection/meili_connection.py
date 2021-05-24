import meilisearch
import json
import os
# utils.environment swaps environment variables on import
import utils.environment

MEILI_HOST = os.getenv("MEILI_HOST")
MEILI_PORT = os.getenv("MEILI_PORT")
MEILI_MASTER_KEY = os.getenv("MEILI_MASTER_KEY")

def make_meili_pool():
    return meilisearch.Client(f"http://{MEILI_HOST}:{MEILI_PORT}")

