### Running:

Use these commands (In this order):
> docker-compose -f docker-compose.db.yml up \
> docker-compose -f docker-compose.data-processing.yml up \
> docker-compose -f docker-compose.main.yml up

Run this command:

> docker-compose up --build

The entire project's total size is 8~ gb.
It'll probably take 10 minutes total. Don't stop the process prematurely. 
It's done when you see this guy: ヽ༼ຈل͜ຈ༽ﾉ



Use -d for detached mode. \
Or start a new terminal with each command. \
Note that data-population shuts down immediately after having finished its purpose. \
Note that the Rust backend container's compilation time is very slow. \
Note that I understand, that using multiple docker-compose files is suboptimal \
You will get messages from your terminal stating, that there are orphan containers.\
It's due to time constraints. I need to use wait-for-it.sh files or similar solutions. \
That takes too long time to set up. \
 \
Containers: \
localhost:31290 - Frontend \
localhost:31291 - Backend \
localhost:31292 - Data processing(shuts down immediately) \
localhost:31293 - Mongodb \
localhost:31294 - Meilisearch \
localhost:31295 - Postgres \
localhost:31296 - Redis \
localhost:31297 - Redis-commander \


### Running locally without containers:

#### Dependencies

#### Required

Functional postgres setup on the machine. \
    Required for psycopg2 \
    Linux machines: Need libpg-devel \
        On fedora: \
            > sudo dnf install postgresql-devel \
Python 3.9 \
C Compiler for Cython (Like gcc). \
Rust 1.52.1

#### Optional

Poetry \
