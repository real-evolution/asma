CREATE TABLE account_roles
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,

    account_id UUID NOT NULL,
    role_id UUID NOT NULL,
    is_active BOOLEAN DEFAULT true NOT NULL,

    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,

    CONSTRAINT account_roles_unique UNIQUE (account_id, role_id),

    CONSTRAINT account_fk FOREIGN KEY (account_id)
                          REFERENCES accounts(id)
                          ON DELETE CASCADE,
    CONSTRAINT role_fk FOREIGN KEY (role_id)
                       REFERENCES roles(id)
                       ON DELETE CASCADE
);
