-- Add up migration script here

alter table work_logs drop column parent_id;
alter table work_logs drop column status;

create table work_logs_mapping
(
    parent_id uuid            not null references work_logs (id),
    user_id   uuid            not null references users (id),
    status    work_log_status not null,
    primary key (parent_id, user_id)
);
