-- Add down migration script here

alter table users
    drop column nickname,
    drop column deleted_at,
    drop column updated_at;
