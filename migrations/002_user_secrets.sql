create table user_secrets (
  user_id uuid not null primary key references users on delete cascade,
  password_hash text,
  last_login_at timestamptz not null default now(),
  failed_password_attempts int not null default 0,
  first_failed_password_attempt timestamptz,
  reset_password_token text,
  reset_password_token_generated timestamptz,
  failed_reset_password_attempts int not null default 0,
  first_failed_reset_password_attempt timestamptz,
  delete_account_token text,
  delete_account_token_generated timestamptz
);
comment on table user_secrets is
  E'The contents of this table should never be visible to the user. Contains data mostly related to authentication.';

create function insert_user_secret_for_user() returns trigger as $$
begin
  insert into user_secrets(user_id) values(NEW.id);
  return NEW;
end;
$$ language plpgsql;
comment on function insert_user_secret_for_user() is
  E'Ensures that every user record has an associated user_secret record.';

create trigger insert_user_secrets
  after insert on users
  for each row
  execute procedure insert_user_secret_for_user();
