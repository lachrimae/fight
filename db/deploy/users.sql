-- Deploy fight:users to pg
-- requires: appschema

BEGIN;

  create table fight.user (
    id uuid primary key default gen_random_uuid(),
    created_at timestamptz not null default now(),
    modified_at timestamptz not null default now()
  );

  create function update_user_timestamp() returns trigger as $$
  begin
    new.modified_at = now();
    return new;
  end; 
  $$ language plpgsql;

  create trigger update_user_timestamp
  after update on fight.user
    for each row
  execute procedure update_user_timestamp();

COMMIT;
