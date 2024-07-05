-- Add migration script here
ALTER TABLE users ADD email TEXT NOT NULL DEFAULT '';
ALTER TABLE users ADD password TEXT NOT NULL DEFAULT '';
