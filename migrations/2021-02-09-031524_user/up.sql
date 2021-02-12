-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL,
    username TEXT NOT NULL UNIQUE,
    login_status BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY (id)
);
