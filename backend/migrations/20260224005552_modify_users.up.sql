-- Add up migration script here

alter table users
    add column nickname varchar(255);
