-- Add migration script here
CREATE TABLE user_history (
  id SERIAL PRIMARY KEY,
  userid SERIAL NOT NULL,
  content TEXT NOT NULL,
  timeof TIMESTAMP NOT NULL,
  CONSTRAINT fk_user FOREIGN KEY(userid) REFERENCES history(id)
)