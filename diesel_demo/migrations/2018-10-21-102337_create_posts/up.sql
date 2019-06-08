-- Your SQL goes here
CREATE TABLE posts (
  id integer PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  owner integer NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE usr (
  id integer PRIMARY KEY,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL,
  mail VARCHAR NOT NULL,
  owner integer NOT NULL
);
