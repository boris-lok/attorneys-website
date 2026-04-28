-- Add up migration script here

alter table cases add column started_at timestamptz;
alter table cases add column ended_at timestamptz;
