
##
For development purposes, the backend can be run with:
> python src/main.py

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