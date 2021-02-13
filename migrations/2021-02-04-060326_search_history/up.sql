-- Your SQL goes here
CREATE TABLE IF NOT EXISTS histories (
    id SERIAL,
    userid INTEGER
    input TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY (id)
);
