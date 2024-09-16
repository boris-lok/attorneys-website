-- Add up migration script here

create table member
(
    id         varchar(32) not null,
    created_at timestamptz not null,
    deleted_at timestamptz,
    seq smallint not null,
    primary key (id)
);

create table content
(
    id         varchar(32) not null,
    data       jsonb       not null,
    created_at timestamptz not null,
    updated_at timestamptz,
    language   char(4)     not null,
    primary key (id, language)
);

create table avatar
(
    id   varchar(32) not null,
    data jsonb       not null,
    primary key (id)
);
