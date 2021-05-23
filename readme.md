Dependencies:

Required:
Functional postgres setup on the machine.
    Required for psycopg2
    Linux machines: Need libpg-devel
        On fedora:
            > sudo dnf install postgresql-devel
Python 3.9

Optional:
Poetry

data_preparation uses Cython. Running program outside docker requires C compiler.

A fully containerized solution could be preferable.
For now it's outside the scope of this project.

