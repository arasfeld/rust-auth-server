create table user_authentications (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users on delete cascade,
  service text not null,
  identifier text not null,
  details jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  constraint uniq_user_authentications unique(service, identifier)
);
comment on table user_authentications is
  E'Contains information about the login providers this user has used, so that they may disconnect them should they wish.';
comment on column user_authentications.service is
  E'The login service used, e.g. `google` or `twitter`.';
comment on column user_authentications.identifier is
  E'A unique identifier for the user within the login service.';
comment on column user_authentications.details is
  E'Additional profile details extracted from this login method';

create index on user_authentications(user_id);

create trigger update_timestamps_user_authentications
  before insert or update on user_authentications
  for each row
  execute procedure set_timestamps();
