CREATE TABLE sessions
(
    id UUID NOT NULL PRIMARY KEY,

    device_identifier VARCHAR NOT NULL,
    agent VARCHAR NOT NULL,
    last_address VARCHAR NULL,

    user_id UUID NOT NULL,
    account_id UUID NOT NULL,

    last_access TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    valid_until TIMESTAMPTZ NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT account_fk FOREIGN KEY (account_id) REFERENCES accounts(id)
);
