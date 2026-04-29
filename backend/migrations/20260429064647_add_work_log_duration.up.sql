-- Add up migration script here

alter table work_logs
    add column duration_minutes int
        generated always as (
    (extract (epoch from ended_at - started_at) / 60)::int
            ) stored;
