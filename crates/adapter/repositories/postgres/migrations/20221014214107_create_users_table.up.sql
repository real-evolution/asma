CREATE TABLE users
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    username VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    is_active BOOLEAN DEFAULT FALSE NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE INDEX users_created_at_idx ON users USING btree (created_at);
