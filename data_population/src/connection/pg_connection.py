import psycopg2
import os
import misc.environment

POSTGRES_USER = os.getenv('POSTGRES_USER')
POSTGRES_PASS = os.getenv('POSTGRES_PASSWORD')
POSTGRES_DATABASE = os.getenv('POSTGRES_DB')
POSTGRES_HOST = os.getenv('POSTGRES_HOST')
POSTGRES_PORT = os.getenv('POSTGRES_PORT')

def make_pg_pool():
    return psycopg2.connect(
        database=POSTGRES_DATABASE,
        user=POSTGRES_USER,
        password=POSTGRES_PASSWORD,
        host=POSTGRES_HOST,
        port=POSTGRES_PORT
    )
