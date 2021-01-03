-- Your SQL goes here
CREATE TABLE IF NOT EXISTS history (
    id SERIAL,
    input TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (id)
);
