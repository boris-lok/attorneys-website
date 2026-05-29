-- Add down migration script here
alter table work_logs drop column settled_date;
