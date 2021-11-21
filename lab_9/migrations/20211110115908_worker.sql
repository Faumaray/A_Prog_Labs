CREATE TABLE workers (
  id SERIAL PRIMARY KEY,
  fname VARCHAR NOT NULL,
  manager TEXT NOT NULL,
  salary INTEGER NOT NULL,
  div_num INTEGER NOT NULL
)
-- Add migration script here
