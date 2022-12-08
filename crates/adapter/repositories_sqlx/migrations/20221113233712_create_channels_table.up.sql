CREATE TABLE channels
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    name VARCHAR NOT NULL,
    platform INTEGER NOT NULL,
    api_key VARCHAR NOT NULL,
    valid_until TIMESTAMPTZ NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    max_instances INT8 NULL,
    user_id UUID NOT NULL,

    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT user_fk FOREIGN KEY (user_id)
                       REFERENCES users(id)
                       ON DELETE CASCADE
);

CREATE INDEX channels_created_at_idx ON channels USING btree (created_at);
