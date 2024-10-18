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

CREATE TABLE users
(
    id            uuid NOT NULL,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    PRIMARY KEY (id)
);

insert into users
    (id, username, password_hash)
values ('47ff8e18-7732-4e8c-a377-ec7491bd93d1', 'boris',
        '$argon2d$v=19$m=15000,t=2,p=1$++wL1wKozweyizKVauMbFQ$XsZbPyHX3I+ndHRz8pWK+ltETNVBMbRzMoE5A2HOqqw');
