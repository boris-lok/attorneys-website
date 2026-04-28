-- Add down migration script here

alter table cases drop column started_at;
alter table cases drop column ended_at;
