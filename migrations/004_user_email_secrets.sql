create table user_email_secrets (
  user_email_id uuid primary key references user_emails on delete cascade,
  verification_token text,
  verification_email_sent_at timestamptz,
  password_reset_email_sent_at timestamptz
);
comment on table user_email_secrets is
  E'The contents of this table should never be visible to the user. Contains data mostly related to email verification and avoiding spamming users.';
comment on column user_email_secrets.password_reset_email_sent_at is
  E'We store the time the last password reset was sent to this email to prevent the email getting flooded.';

create function insert_user_email_secret_for_user_email() returns trigger as $$
declare
  v_verification_token text;
begin
  if NEW.is_verified is false then
    v_verification_token = encode(gen_random_bytes(7), 'hex');
  end if;
  insert into user_email_secrets(user_email_id, verification_token) values(NEW.id, v_verification_token);
  return NEW;
end;
$$ language plpgsql;
comment on function insert_user_email_secret_for_user_email() is
  E'Ensures that every user_email record has an associated user_email_secret record.';

create trigger insert_user_email_secrets
  after insert on user_emails
  for each row
  execute procedure insert_user_email_secret_for_user_email();
