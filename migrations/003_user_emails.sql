create table user_emails (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users on delete cascade,
  email varchar(255) not null check (email ~ '[^@]+@[^@]+\.[^@]+'),
  is_verified boolean not null default false,
  is_primary boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  -- Each user can only have an email once.
  constraint user_emails_user_id_email_key unique(user_id, email),
  -- An unverified email cannot be set as the primary email.
  constraint user_emails_must_be_verified_to_be_primary check(is_primary is false or is_verified is true)
);
comment on table user_emails is
  E'Information about a user''s email address.';
comment on column user_emails.email is
  E'The users email address, in `a@b.c` format.';
comment on column user_emails.is_verified is
  E'True if the user has is_verified their email address (by clicking the link in the email we sent them, or logging in with a social login provider), false otherwise.';

create unique index uniq_user_emails_verified_email on user_emails (email) where (is_verified is true);
create unique index uniq_user_emails_primary_email on user_emails (user_id) where (is_primary is true);
create index idx_user_emails_user on user_emails (user_id);
create index idx_user_emails_primary on user_emails (is_primary, user_id);
