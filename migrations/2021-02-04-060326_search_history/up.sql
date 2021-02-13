-- Your SQL goes here
CREATE TABLE IF NOT EXISTS history (
    id SERIAL,
    userid INTEGER NOT NULL,
    input TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY (id)
);
