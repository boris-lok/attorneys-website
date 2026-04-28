-- Add up migration script here

alter table cases add column started_at timestampz;
alter table cases add column ended_at timestampz;
