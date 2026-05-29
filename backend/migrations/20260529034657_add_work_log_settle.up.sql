-- Add up migration script here
alter table work_logs add column settled_at timestamptz;
