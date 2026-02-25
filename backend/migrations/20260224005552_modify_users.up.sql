-- Add up migration script here

alter table users
    add column nickname varchar(255),
    add column deleted_at timestamptz,
    add column updated_at timestamptz;
