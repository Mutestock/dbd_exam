import click

from logic import mongo_populate
from logic import meili_populate
from logic import pg_populate
import utils.environment
import time

@click.group()
def manager():
    pass

@manager.command()
@click.option(
    "--mongo",
    "-mo",
    is_flag=True,
    help="Re(populates) mongo db",
)
@click.option(
    "--meili",
    "-me",
    is_flag=True,
    help="Re(populates) meilisearch db",
)
@click.option(
    "--postgres",
    "-pg",
    is_flag=True,
    help="Re(populates) postgresql db",
)
def populate(mongo, meili, postgres):
    if mongo:
        mongo_populate.generate_universities()
        mongo_populate.generate_locations()
        mongo_populate.generate_people()
    if meili:
        meili_populate.populate_meili_locations()
        time.sleep(15)
        meili_populate.populate_meili_university()
        time.sleep(15)
        meili_populate.populate_meili_people()
    if postgres:
        pg_populate.reset_tables()
        pg_populate.mass_populate()

if __name__ == "__main__":
    manager()
