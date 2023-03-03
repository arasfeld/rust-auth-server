create extension if not exists "pgcrypto";

create or replace function set_timestamps() returns trigger as $$
begin
    NEW.created_at = (case when TG_OP = 'INSERT' then NOW() else OLD.created_at end);
    NEW.updated_at = (case when TG_OP = 'UPDATE' and OLD.updated_at >= NOW() then OLD.updated_at + interval '1 millisecond' else NOW() end);
    return NEW;
end;
$$ language plpgsql;
comment on function set_timestamps() is
  E'This trigger should be called on all tables with created_at, updated_at - it ensures that they cannot be manipulated and that updated_at will always be larger than the previous updated_at.';
