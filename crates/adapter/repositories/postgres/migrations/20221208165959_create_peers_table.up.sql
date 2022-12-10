CREATE TABLE peers
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    display_name VARCHAR NULL,
    comment VARCHAR NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    user_id UUID NOT NULL,

    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT user_fk FOREIGN KEY (user_id)
                       REFERENCES users(id)
                       ON DELETE CASCADE
);

CREATE INDEX peers_created_at_idx ON channels USING btree (created_at);
