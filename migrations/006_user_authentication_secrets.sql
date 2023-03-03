create table user_authentication_secrets (
  user_authentication_id uuid not null primary key references user_authentications on delete cascade,
  details jsonb not null default '{}'::jsonb
);
comment on table user_authentication_secrets is
  E'The contents of this table should never be visible to the user. Contains data mostly related to authentication.';
comment on column user_authentication_secrets.details is
  E'Contains things like access tokens, refresh tokens, profile information, etc.';
