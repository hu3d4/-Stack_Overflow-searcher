-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL,
    email TEXT NOT NULL,
    pw TEXT NOT NULL,
    login_status BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY (id)
);
