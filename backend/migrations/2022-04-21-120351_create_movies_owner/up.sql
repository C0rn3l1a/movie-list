-- Your SQL goes here
CREATE TABLE IF NOT EXISTS owners
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS movies
(
    id SERIAL PRIMARY KEY NOT NULL,
    owner_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    seen BOOLEAN NOT NULL DEFAULT 'f',

    CONSTRAINT fk_movies_owners_id FOREIGN KEY (owner_id) REFERENCES movies(id)
);