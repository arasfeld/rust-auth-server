/*
 * In addition to logging in with username/email and password, users may use
 * other authentication methods, such as "social login" (OAuth) with GitHub,
 * Twitter, Facebook, etc. We store details of these logins to the
 * user_authentications table.
 */
create table user_authentications (
  id uuid primary key default uuid_generate_v4(),
  user_id uuid not null references users on delete cascade,
  service text not null,
  identifier text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  constraint uniq_user_authentications unique(service, identifier)
);

-- Make it efficient to find all the authentications for a particular user.
create index on user_authentications(user_id);

-- Keep created_at and updated_at up to date.
create trigger update_timestamps_user_authentications
  before insert or update on user_authentications
  for each row
  execute procedure set_timestamps();
