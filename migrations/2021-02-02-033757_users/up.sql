-- Your SQL goes here
CREATE TABLE users (
      id SERIAL PRIMARY KEY,
      name VARCHAR NOT NULL,
      is_male BOOLEAN NOT NULL DEFAULT 'f'
)
