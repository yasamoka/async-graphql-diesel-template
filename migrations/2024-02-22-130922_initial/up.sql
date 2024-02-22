CREATE TYPE binding AS ENUM ('HARDCOVER', 'PAPERBACK');

CREATE TABLE author(
    id SERIAL NOT NULL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);

CREATE TABLE book(
    id SERIAL NOT NULL PRIMARY KEY,
    author_id INTEGER NOT NULL,
    isbn TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    binding binding NOT NULL,
    FOREIGN KEY(author_id) REFERENCES author(id)
);