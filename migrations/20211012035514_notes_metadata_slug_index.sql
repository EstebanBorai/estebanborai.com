-- Add migration script here
CREATE UNIQUE INDEX slug_index ON notes_metadata(slug);
