-- Add migration script here
CREATE TABLE history (
  id SERIAL PRIMARY KEY NOT NULL,
  client_name VARCHAR NOT NULL
)
