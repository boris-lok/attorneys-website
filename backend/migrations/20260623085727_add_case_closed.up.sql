-- Add up migration script here

alter table cases add column closed boolean default false;
