CREATE TYPE binding AS ENUM ('HARDCOVER', 'PAPERBACK');

CREATE TABLE author(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);

CREATE TABLE book(
    id SERIAL NOT NULL PRIMARY KEY,
    author_id uuid NOT NULL,
    isbn TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    binding binding NOT NULL,
    FOREIGN KEY(author_id) REFERENCES author(id)
);