CREATE TABLE instances
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    platform_identifier VARCHAR NOT NULL,

    display_name VARCHAR NULL,
    phone_number VARCHAR NULL,

    last_active TIMESTAMPTZ NULL,
    is_default BOOLEAN DEFAULT FALSE NOT NULL,

    channel_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT channel_fk FOREIGN KEY (channel_id) REFERENCES channels(id)
);

CREATE INDEX instances_created_at_idx ON channels USING btree (created_at);
