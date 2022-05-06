DROP DATABASE IF EXISTS todo;
DROP USER IF EXISTS todo_user;
DROP TABLE IF EXISTS todos;

CREATE DATABASE todo;

CREATE USER todo_user WITH PASSWORD 'todo_password';
GRANT ALL PRIVILEGES ON DATABASE todo to todo_user;

CREATE TABLE todos (
        id BIGSERIAL,
        title   TEXT NOT NULL,
        description TEXT NOT NULL,
        status TEXT DEFAULT 'OPEN',
        created_by BIGINT,
        created BIGINT
);

ALTER SEQUENCE todos_id_seq RESTART WITH 1000;

INSERT INTO todos values (100, 'Todo 01', 'This is todo 01', 'OPEN', '01', '01');

INSERT INTO todos values (101, 'Todo 02', 'This is todo 02', 'OPEN', '02', '02');


