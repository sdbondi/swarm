-- Add migration script here

CREATE TABLE processes
(
    id            integer primary key autoincrement,
    instance_id   BIGINT       NOT NULL,
    instance_name VARCHAR(255) NOT NULL,
    pid           INT          NULL,
    created_at    TIMESTAMP    NOT NULL DEFAULT current_timestamp
);