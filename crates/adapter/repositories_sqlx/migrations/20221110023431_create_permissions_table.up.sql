CREATE TABLE permissions
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    resource BIGINT NOT NULL,
    actions INT8 NOT NULL,
    role_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,

    CONSTRAINT role_fk FOREIGN KEY (role_id) REFERENCES roles(id)
);
