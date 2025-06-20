-- storing the config like this is not the ideal, but I already have a database
-- and I don't want to deal with creating, serializing... a config file
CREATE TABLE config (
 id INTEGER PRIMARY KEY CHECK (id = 1),
 gotenberg_location TEXT NULL
);

INSERT INTO config (id, gotenberg_location) VALUES (1, NULL);