/* 
 * This extension gives us `uuid_generate_v1mc()` which generates UUIDs that cluster better than `gen_random_uuid()`
 * while still being difficult to predict and enumerate.
 * Also, while unlikely, `gen_random_uuid()` can in theory produce collisions which can trigger spurious errors on
 * insertion, whereas it's much less likely with `uuid_generate_v1mc()`.
 */
create extension if not exists "uuid-ossp";

/*
 * This trigger is used on tables with created_at and updated_at to ensure that
 * these timestamps are kept valid (namely: `created_at` cannot be changed, and
 * `updated_at` must be monotonically increasing).
 */
create or replace function set_timestamps() returns trigger as $$
begin
    NEW.created_at = (case when TG_OP = 'INSERT' then NOW() else OLD.created_at end);
    NEW.updated_at = (case when TG_OP = 'UPDATE' and OLD.updated_at >= NOW() then OLD.updated_at + interval '1 millisecond' else NOW() end);
    return NEW;
end;
$$ language plpgsql;
