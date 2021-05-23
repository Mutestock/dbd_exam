import importlib
import unittest
import os

SRC_PATH = "../src"

connections = importlib.import_module("connection", SRC_PATH)
environment = importlib.import_module("misc.environment", SRC_PATH)


class TestMisc(unittest.TestCase):
    def test_environment_variable_file_found(self):
        self.assertTrue(os.path.isfile(environment.ENVIRONMENT_VARIABLE_FILE))

class TestMongo(unittest.TestCase):
    def test_mongo_connection(self):
        connections.mongo_connection.make_mongo_pool()
        
class TestPostgres(unittest.TestCase):
    def test_postgres_connection(self):
        connection = connections.postgres_connection.make_pg_pool()
        cursor = connection.cursor()
        self.assertTrue(cursor is not None)
