CREATE TABLE bots
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    name VARCHAR NOT NULL UNIQUE,
    is_active BOOLEAN DEFAULT FALSE NOT NULL,
    user_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT user_fk FOREIGN KEY (user_id)
                       REFERENCES users(id)
                       ON DELETE CASCADE
);

CREATE INDEX bots_created_at_idx ON bots USING btree (created_at);
