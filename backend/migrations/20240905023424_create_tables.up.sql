-- Add up migration script here

create table resource
(
    id            varchar(32) not null,
    created_at    timestamptz not null,
    deleted_at    timestamptz,
    resource_type varchar(32) not null,
    seq           smallint    not null,
    primary key (id)
);

create table content
(
    id         varchar(32) not null,
    data       jsonb       not null,
    created_at timestamptz not null,
    updated_at timestamptz,
    language   varchar(8)  not null,
    primary key (id, language)
);

create table avatar
(
    id   varchar(32) not null,
    data jsonb       not null,
    primary key (id)
);
