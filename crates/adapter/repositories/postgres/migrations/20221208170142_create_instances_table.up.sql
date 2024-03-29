CREATE TABLE instances
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    platform_identifier BIGINT NOT NULL,

    username VARCHAR NULL,
    display_name VARCHAR NULL,
    phone_number VARCHAR NULL,

    last_active TIMESTAMPTZ NULL,

    chat_id UUID NOT NULL,
    channel_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT chat_fk UNIQUE (chat_id),
    CONSTRAINT channel_fk FOREIGN KEY (channel_id)
                          REFERENCES channels(id)
                          ON DELETE CASCADE
);

CREATE INDEX instances_created_at_idx ON channels USING btree (created_at);
CREATE INDEX instances_platform_identifier_idx ON channels USING btree (created_at);
CREATE INDEX instances_platform_username_idx ON channels USING btree (created_at);
