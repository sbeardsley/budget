-- Add migration script here
ALTER TABLE budgets
    ADD COLUMN user_id TEXT NOT NULL;
