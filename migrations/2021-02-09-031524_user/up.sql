-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL,
    email TEXT NOT NULL,
    pw TEXT NOT NULL,
    PRIMARY KEY (id)
);
