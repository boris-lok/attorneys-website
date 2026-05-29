-- Add up migration script here
alter table cases add column settled_at timestamptz;
alter table cases add column billing_cycle integer;
