CREATE TABLE menus
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    title VARCHAR NOT NULL,
    content VARCHAR NULL,
    menu_trigger VARCHAR NOT NULL,
    matching_strategy INTEGER NOT NULL,

    is_active BOOLEAN DEFAULT FALSE NOT NULL,
    parent_menu_id UUID NOT NULL,
    bot_id UUID NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT parent_menu_fk FOREIGN KEY (parent_menu_id)
                              REFERENCES menus(id)
                              ON DELETE CASCADE,
    CONSTRAINT bot_fk FOREIGN KEY (bot_id)
                      REFERENCES bots(id)
                      ON DELETE CASCADE
);

CREATE INDEX menus_created_at_idx ON menus USING btree (created_at);
