### Running:

Run this command:

> docker-compose up --build

The entire project's total size is 8~ gb. \
It'll probably take 10 minutes total. Don't stop the process prematurely. \
3~ minutes of those 10~ comes from the data-population container. \
The entire process is done when you see this guy: ヽ༼ຈل͜ຈ༽ﾉ 



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
localhost:11290 - Frontend \
localhost:11291 - Backend \
localhost:11292 - Data processing(shuts down after use) \
localhost:11293 - Mongodb \
localhost:11294 - Meilisearch \
localhost:11295 - Postgres \
localhost:11296 - Redis \
localhost:11297 - Redis-commander 


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

... And you'd also need all of the databases set up.

#### Optional

Poetry 
