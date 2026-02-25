-- Add up migration script here

create type work_log_status as enum('pending', 'rejected', 'approved');

alter table work_logs
    add column status work_log_status not null;
