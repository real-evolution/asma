CREATE TABLE accounts
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    account_name VARCHAR NOT NULL,
    holder_name VARCHAR NULL,

    password_hash VARCHAR NOT NULL,
    valid_until TIMESTAMPTZ NULL,
    is_active BOOLEAN DEFAULT FALSE NOT NULL,

    user_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES users(id)
);
