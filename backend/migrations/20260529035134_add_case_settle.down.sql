-- Add down migration script here
alter table cases drop column settled_date;
alter table cases drop column billing_cycle;
