CREATE TABLE accounts
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    account_name VARCHAR NOT NULL,
    holder_name VARCHAR NULL,

    password_hash VARCHAR NOT NULL,
    state INTEGER DEFAULT 0 NOT NULL,
    user_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT user_fk FOREIGN KEY (user_id)
                       REFERENCES users(id)
                       ON DELETE CASCADE
);

CREATE INDEX accounts_created_at_idx ON accounts USING btree (created_at);
