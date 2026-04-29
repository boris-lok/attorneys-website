-- Add down migration script here

alter table work_logs drop column duration_minutes;
