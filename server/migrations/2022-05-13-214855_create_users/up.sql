CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  pw_hash TEXT NOT NULL
);

INSERT INTO users VALUES (1, 'dummy@gmail.com', 'dumbdummy', 'TODO_insert_valid_pw_hash'); /* TODO actual password */