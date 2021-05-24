import importlib
import unittest
import os
import json

SRC_PATH = "../src"

connections = importlib.import_module("connection", SRC_PATH)
meili_connection = importlib.import_module("connection.meili_connection", SRC_PATH)
mongo_connection = importlib.import_module("connection.mongo_connection", SRC_PATH)
postgres_connection = importlib.import_module("connection.pg_connection", SRC_PATH)
environment = importlib.import_module("utils.environment", SRC_PATH)


class TestMisc(unittest.TestCase):
    def test_environment_variable_file_found(self):
        self.assertTrue(os.path.isfile(environment.ENVIRONMENT_VARIABLE_FILE))


class TestMongo(unittest.TestCase):
    def test_mongo_connection(self):
        connection = mongo_connection.make_mongo_pool()
        self.assertTrue(connection is not None)


class TestPostgres(unittest.TestCase):
    def test_postgres_connection(self):
        connection = postgres_connection.make_pg_pool()
        cursor = connection.cursor()
        self.assertTrue(cursor is not None)


class TestMeili(unittest.TestCase):
    def test_meili_connection(self):
        connection = meili_connection.make_meili_pool()
        self.assertTrue(connection is not None)

    def test_meili_health(self):
        connection = meili_connection.make_meili_pool()
        self.assertTrue(connection.health() is not None)

    def test_meili_create_index(self):
        connection = meili_connection.make_meili_pool()
        try:
            connection.index("test").delete()
        except:
            pass
        connection.create_index("test", {"primaryKey": "id"})
        connection.index("test").delete()

    def test_meili_add_basic_test_document(self):
        connection = meili_connection.make_meili_pool()
        try:
            connection.index("test").delete()
        except:
            pass

        connection.create_index("test", {"primaryKey": "id", "name": "title"})
        connection.index("test").add_documents([{"id": 999999, "title": "cake"}])
        connection.index("test").delete()

    def test_meili_add_get_basic_test_document(self):
        connection = meili_connection.make_meili_pool()
        try:
            connection.index("test").delete()
        except:
            pass
        connection.create_index("test", {"primaryKey": "id", "name": "title"})
        connection.index("test").add_documents([{"id": 999999, "title": "cake"}])
        self.assertTrue(connection.index("test").search("test_document") is not None)
        connection.index("test").delete()
