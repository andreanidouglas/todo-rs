-- Add migration script here
CREATE TABLE todos(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    completed BOOLEAN NOT NULL,
    created_at timestamptz NOT NULL
);
