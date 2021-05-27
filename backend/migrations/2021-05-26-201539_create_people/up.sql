-- Your SQL goes here

-- This table will be created with SQLAlchemy in data_processing as well.

CREATE TABLE IF NOT EXISTS people (
    id SERIAL PRIMARY KEY NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(500) NOT NULL,
    phone_number VARCHAR(50) NOT NULL,
    avatar VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


