create table account_roles
(
    id         uuid                      not null primary key,
    account_id uuid                      null,
    role_id    uuid                      not null,
    enabled    boolean     default true  not null,
    created_at timestamptz default now() not null,

    constraint account_roles_account_fk
      foreign key (account_id)
        references accounts(id),

    constraint account_roles_role_fk
      foreign key (role_id)
        references roles(id),

    constraint account_roles_unique
      unique (account_id, role_id)
);
