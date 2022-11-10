CREATE TABLE account_roles
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    account_id UUID NULL,
    role_id UUID NOT NULL,
    enabled BOOLEAN DEFAULT true NOT null,

    created_at TIMESTAMPTZ DEFAULT now() NOT null,

    CONSTRAINT account_roles_unique UNIQUE (account_id, role_id),
    CONSTRAINT account_fk FOREIGN KEY (account_id) REFERENCES accounts(id),
    CONSTRAINT role_fk FOREIGN KEY (role_id) REFERENCES roles(id)
);
