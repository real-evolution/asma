create table roles
(
    id            uuid                      not null primary key,
    code          varchar                   not null unique,
    friendly_name varchar                   null,
    created_at    timestamptz default now() not null,
    updated_at    timestamptz default now() not null
);
