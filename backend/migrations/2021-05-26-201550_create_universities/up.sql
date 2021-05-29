-- Your SQL goes here

-- This table will be created in data_processing as well.

CREATE TABLE IF NOT EXISTS universities(
    id SERIAL PRIMARY KEY NOT NULL,
    country_index VARCHAR(50) NOT NULL,
    university_name VARCHAR(500) NOT NULL,
    website_url VARCHAR(500) NOT NULL,
    locations_id INTEGER REFERENCES locations
);