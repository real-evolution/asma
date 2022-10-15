create table users
(
    id           uuid                      not null primary key,
    username     varchar                   not null unique,
    display_name varchar                   not null,
    state        integer     default 0     not null,
    created_at   timestamptz default now() not null,
    updated_at   timestamptz default now() not null
);
