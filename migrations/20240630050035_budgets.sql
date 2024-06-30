-- Add migration script here
CREATE TABLE budgets (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    total REAL NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_budgets_updated_at
AFTER UPDATE ON budgets
FOR EACH ROW
BEGIN
    UPDATE budgets
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
