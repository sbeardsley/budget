-- Add migration script here
CREATE TABLE categories (
    id TEXT NOT NULL PRIMARY KEY,
    budget_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(budget_id) REFERENCES budgets(id)
);

CREATE TRIGGER update_categories_updated_at
AFTER UPDATE ON categories
FOR EACH ROW
BEGIN
    UPDATE categories
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
