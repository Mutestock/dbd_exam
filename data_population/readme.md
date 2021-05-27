
If not using Docker, the following is required:

python 3.9 (All others are untested)

[poetry](https://python-poetry.org/) 
or
[Some other python virtual environment](https://docs.python.org/3/library/venv.html#creating-virtual-environments)

C compiler for Cython
Linux:

> dnf install gcc (Or your distribution's equivalent) \
[minGW for Windows?](http://mingw.osdn.io/)

Linux: 
libpg-devel (or your distribution's equivalent (postgresql-devel on fedora) )

Others:
Possibly more dependencies. I'm using Linux, so it's hard for me to certain

If you're not using poetry - after having activated your virtual environment:

>pip install -r requirements.txt

Otherwise use guide below

##
For development purposes, the backend can be run with:

>python src/main.py

## Poetry guide

cd to this directory
### To enter environment:
>poetry shell

### Add packages:
>poetry add insert_your_package_here

### Remove packages:
>poetry remove insert_your_package_here

### Whenever you've added or removed packages: 
>poetry export -f requirements.txt --output requirements.txt 

This requirements.txt is like a recipe for the required modules. It's so the server's container installs the correct packages. It's also required for people who use venv.

### Whenever someone else has made changes and you need to update your shell: 
>poetry update

### Run tests with:
>nose2 -v