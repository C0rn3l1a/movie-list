-- This file should undo anything in `up.sql`
ALTER TABLE movies DROP CONSTRAINT "fk_movies_owners_id";
DROP TABLE movies;
DROP TABLE owners;