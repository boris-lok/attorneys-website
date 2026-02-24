-- Add up migration script here

create table cases (
    id uuid primary key,
    name text not null,
    estimated_minutes integer not null check ( estimated_minutes > 0 ),
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    deleted_at timestamptz
);
