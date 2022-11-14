CREATE TABLE sessions
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    device_identifier VARCHAR NOT NULL,
    agent VARCHAR NOT NULL,
    refresh_token VARCHAR NOT NULL,
    last_address VARCHAR NOT NULL,
    account_id UUID NOT NULL,

    expires_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT account_fk FOREIGN KEY (account_id)
                          REFERENCES accounts(id)
                          ON DELETE CASCADE
);
