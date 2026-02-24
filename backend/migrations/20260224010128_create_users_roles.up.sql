-- Add up migration script here

create table roles
(
    id   uuid primary key default gen_random_uuid(),
    name text not null unique
);

create table user_roles
(
    user_id uuid not null references users (id),
    role_id uuid not null references roles (id),
    primary key (user_id, role_id)
);
