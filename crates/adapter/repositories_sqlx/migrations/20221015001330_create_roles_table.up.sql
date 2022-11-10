CREATE TABLE roles
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    code VARCHAR NOT NULL UNIQUE,
    friendly_name VARCHAR NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);
