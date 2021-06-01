### Description:

This is the project is my answer for the DBD exam. It is a one-man project.\
If you want to know more, I encourage the reader to look in ./documentation/latex/report.pdf

### Running:

Run this command:

> docker-compose up --build

The entire project's total size is 6~ gb. \
It'll probably take 10 minutes total. Don't stop the process prematurely. \
3~ minutes of those 10~ comes from the data-population container. \
The entire process is done when the data-population container exits with code 0.

 \
Containers: \
localhost:11290 - Frontend \
localhost:11291 - Backend \
localhost:11292 - Data processing(shuts down after use) \
localhost:11293 - Mongodb \
localhost:11294 - Meilisearch \
localhost:11295 - Postgres \
localhost:11296 - Redis \
localhost:11297 - Redis-commander \
 \

### Available routes: \
 \
GET: localhost:11291/api/location (list of locations limit 10) \
GET/POST/UPDATE/DELETE: localhost:11291/api/location/{some_integer_id} \
GET: localhost:11291/api/location/search/{Some string} \
 \
GET: localhost:11291/api/person (list of people limit 10) \
GET/POST/UPDATE/DELETE: localhost:11291/api/person/{some_integer_id} \
GET: localhost:11291/api/person/search/{Some string} \
 \
GET: localhost:11291/api/university (list of universities limit 10) \
GET/POST/UPDATE/DELETE: localhost:11291/api/university/{some_integer_id} \
GET: localhost:11291/api/university/search/{Some string} \
 
localhost:11297 is a Redis dashboard.
localhost:11294 contains a minimal demonstration of how Meilisearch functions.


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
Rust 1.52.1 \
 
https://redis.io/download \
https://docs.meilisearch.com/learn/getting_started/installation.html \
https://www.postgresql.org/download/ \
https://www.mongodb.com/try/download/community

#### Optional

Poetry 
