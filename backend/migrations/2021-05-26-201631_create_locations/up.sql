-- Your SQL goes here

-- This table will be created with SQLAlchemy in data_processing as well.

CREATE TABLE IF NOT EXISTS locations(
    id SERIAL PRIMARY KEY NOT NULL,
    street_name VARCHAR(500) NOT NULL,
    zip_code VARCHAR(50) NOT NULL,
    country VARCHAR(500) NOT NULL
);