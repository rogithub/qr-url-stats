-- Add migration script here
CREATE TABLE links (
    id TEXT PRIMARY KEY,
    original_url TEXT NOT NULL,
    scans INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);