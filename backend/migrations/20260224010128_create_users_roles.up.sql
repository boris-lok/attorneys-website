-- Add up migration script here

create table roles
(
    id   smallserial primary key,
    name text not null unique
);

create table user_roles
(
    user_id uuid not null references users (id),
    role_id smallserial not null references roles (id),
    primary key (user_id, role_id)
);

insert into roles (name) values ('Admin');
insert into roles (name) values ('Lawyer');
