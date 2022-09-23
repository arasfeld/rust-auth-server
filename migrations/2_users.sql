/*
 * The users table stores (unsurprisingly) the users of our application. You'll
 * notice that it does NOT contain private information such as the user's
 * password or their email address; that's because the users table is seen as
 * public - anyone who can "see" the user can see this information.
 */
create table users (
  id uuid primary key default uuid_generate_v4(),
  username varchar(24) not null unique check(length(username) >= 2 and length(username) <= 24 and username ~ '^[a-zA-Z]([_]?[a-zA-Z0-9])+$'),
  name text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create trigger update_timestamps_users
  before insert or update on users
  for each row
  execute procedure set_timestamps();

/*
 * The users table contains all the public information, but we need somewhere
 * to store private information. In fact, this data is so private that we don't
 * want the user themselves to be able to see it - things like the bcrypted
 * password hash, timestamps of recent login attempts (to allow us to
 * auto-protect user accounts that are under attack), etc.
 */
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

/*
 * When we insert into `users` we _always_ want there to be a matching
 * `user_secrets` entry, so we have a trigger to enforce this:
 */
create function insert_user_secret_for_user() returns trigger as $$
begin
  insert into user_secrets(user_id) values(NEW.id);
  return NEW;
end;
$$ language plpgsql;

create trigger insert_user_secrets
  after insert on users
  for each row
  execute procedure insert_user_secret_for_user();
