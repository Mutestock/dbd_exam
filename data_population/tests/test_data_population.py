import importlib
import unittest
import os
import json

SRC_PATH = "../src"

connections = importlib.import_module("connection", SRC_PATH)
meili_connection = importlib.import_module("connection.meili_connection", SRC_PATH)
mongo_connection = importlib.import_module("connection.mongo_connection", SRC_PATH)
environment = importlib.import_module("utils.environment", SRC_PATH)
aliases = importlib.import_module("utils.aliases", SRC_PATH)
mongo_populate = importlib.import_module("logic.mongo_populate", SRC_PATH)


class TestMisc(unittest.TestCase):
    def test_environment_variable_file_found(self):
        self.assertTrue(os.path.isfile(environment.ENVIRONMENT_VARIABLE_FILE))


class TestMongo(unittest.TestCase):
    def test_mongo_connection(self):
        pool = mongo_connection.make_mongo_pool()
        self.assertTrue(pool is not None)

    def test_mongo_basic(self):
        pool = mongo_connection.make_mongo_pool()
        db = pool["admin"]
        collection = db["test"]
        test = {"thing": "stuff", "text": "some_arbitrary text"}
        test_id = collection.insert_one(test).inserted_id
        self.assertTrue(test_id is not None)

    def test_email_generation(self):
        email_df = mongo_populate.grab_prepared_email_df()
        email = mongo_populate.generate_email(email_df)
        self.assertTrue("@" in email)
        self.assertTrue(len(email.split("@")) == 2)
        
    def test_first_name_generation(self):
        first_name_df = mongo_populate.grab_prepared_first_names_df()
        first_name = mongo_populate.grab_first_name(first_name_df)
        self.assertTrue(first_name is not None)
    
    def test_last_names_generation(self):
        last_names_df = mongo_populate.grab_prepared_last_names_df()
        last_name = mongo_populate.grab_last_name(last_names_df)
        self.assertTrue(last_name is not None)

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
