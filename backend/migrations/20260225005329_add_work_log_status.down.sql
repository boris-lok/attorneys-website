-- Add down migration script here

alter table work_logs drop column status;
drop type work_log_status;
