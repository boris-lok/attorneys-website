-- Add down migration script here

drop table work_logs_mapping;

alter table work_logs
    add column parent_id uuid references work_logs (id);
alter table work_logs
    add column status work_log_status not null;
