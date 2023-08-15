-- Deploy fight:games to pg
-- requires: appschema

BEGIN;

  create type fight."GameState" as enum('Lobbied', 'Started', 'Completed', 'Cancelled');

  create extension if not exists "pgcrypto";

  create table fight.game (
    id uuid primary key default gen_random_uuid(),
    state fight."GameState" not null,
    created_at timestamptz not null default now(),
    modified_at timestamptz not null default now()
  );

  create function fight.update_game_timestamp() returns trigger as $$
  begin
    new.modified_at = now();
    return new;
  end; 
  $$ language plpgsql;

  create trigger update_game_timestamp
  after update on fight.game
    for each row
  execute procedure fight.update_game_timestamp();

COMMIT;
