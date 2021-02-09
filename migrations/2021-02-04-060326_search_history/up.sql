-- Your SQL goes here
CREATE TABLE IF NOT EXISTS histories (
    id SERIAL,
    input TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY (id)
);
