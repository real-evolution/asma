create table accounts
(
    id           uuid                      not null primary key,
    account_name varchar                   not null,
    holder_name  varchar                   null,
    password     varchar                   null,
    is_active    boolean     default false not null,
    valid_until  timestamptz               null,
    user_id      uuid                      not null,
    created_at   timestamptz default now() not null,
    updated_at   timestamptz default now() not null,

    constraint account_user_fk
      foreign key (user_id)
        references users(id)
);
