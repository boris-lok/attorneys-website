-- Add up migration script here

create table members
(
    id         varchar(32) not null,
    avatar_id  varchar(32),
    created_at timestamptz  not null,
    primary key (id)
);

create table content
(
    id         varchar(32) not null,
    data       jsonb       not null,
    created_at timestamptz  not null,
    language   char(4)     not null,
    primary key (id, language)
);
