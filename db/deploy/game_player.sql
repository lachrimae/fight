-- Deploy fight:game_player to pg
-- requires: games
-- requires: users
-- requires: appschema

BEGIN;

  create table fight.game_player (
    id uuid primary key default gen_random_uuid(),
    game_id uuid not null references fight.game (id),
    user_id uuid not null references fight.user (id),
    created_at timestamptz not null default now(),
    modified_at timestamptz not null default now()
  );

  create function fight.update_game_player_timestamp() returns trigger as $$
  begin
    new.modified_at = now();
    return new;
  end;
  $$ language plpgsql;

  create trigger update_game_player_timestamp
  after update on fight.game_player
    for each row
  execute procedure fight.update_game_player_timestamp();

  create unique index game_player_user_id_idx
    on fight.game_player (user_id);

COMMIT;
