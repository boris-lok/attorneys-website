-- Add up migration script here

create table cases (
    id uuid primary key default gen_random_uuid(),
    name text not null,
    created_at timestamptz not null default now(),
    estimated_minutes integer not null check ( estimated_minutes > 0 )
);
