-- Add up migration script here

create table work_logs (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id),
    case_id uuid not null references cases(id),
    started_at timestamptz not null,
    ended_at timestamptz not null,
    description text,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    deleted_at timestamptz,
    shared boolean not null default false,
    parent_id uuid references work_logs(id)
);

CREATE INDEX idx_work_logs_case_id ON work_logs(case_id);
CREATE INDEX idx_work_logs_user_id ON work_logs(user_id);
CREATE INDEX idx_work_logs_started_at ON work_logs(started_at);
CREATE INDEX idx_work_logs_parent_id ON work_logs(parent_id);
