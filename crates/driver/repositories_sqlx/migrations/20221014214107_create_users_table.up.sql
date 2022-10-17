CREATE TABLE users
(
    id UUID NOT NULL PRIMARY KEY,

    username VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    state INTEGER DEFAULT 0 NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);
