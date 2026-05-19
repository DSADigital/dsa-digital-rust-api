CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(32) NOT NULL,
    email VARCHAR(128) NOT NULL UNIQUE,
    role VARCHAR(16) NOT NULL DEFAULT 'user',
    password TEXT NOT NULL,
    token TEXT NOT NULL
);