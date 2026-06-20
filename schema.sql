CREATE TABLE games (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    creator TEXT NOT NULL,
    plays INTEGER NOT NULL DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')))