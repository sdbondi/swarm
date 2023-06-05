-- Add migration script here

CREATE TABLE processes
(
    id         integer primary key autoincrement,
    name       VARCHAR(255) NOT NULL,
    pid        INT          NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT current_timestamp
);