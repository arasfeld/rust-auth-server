/*
 * A user may have more than one email address; this is useful when letting the
 * user change their email so that they can verify the new one before deleting
 * the old one, but is also generally useful as they might want to use
 * different emails to log in versus where to send notifications. Therefore we
 * track user emails in a separate table.
 */
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

-- Once an email is verified, it may only be used by one user. (We can't
-- enforce this before an email is verified otherwise it could be used to
-- prevent a legitimate user from signing up.)
create unique index uniq_user_emails_verified_email on user_emails(email) where (is_verified is true);
-- Only one primary email per user.
create unique index uniq_user_emails_primary_email on user_emails (user_id) where (is_primary is true);
-- Allow efficient retrieval of all the emails owned by a particular user.
create index idx_user_emails_user on user_emails (user_id);
-- For the user settings page sorting
create index idx_user_emails_primary on user_emails (is_primary, user_id);
