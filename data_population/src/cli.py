import click


@click.group()
def manager():
    pass


@manager.command()
@click.option(
    "--force",
    "-f",
    is_flag=True,
    help="Forces database to overwrite database entries even if they already exist",
)
def populater(force):
    if force:
        pass
    else:
        pass


@manager.command()
@click.option(
    "--force",
    "-f",
    help="Forces database to overwrite database entries even if they already exist",
)
def to_pg(force):
    if force:
        pass
    else:
        pass
