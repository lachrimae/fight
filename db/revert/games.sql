-- Revert fight:games from pg

BEGIN;

  drop trigger update_game_timestamp on fight.game;
  drop function update_game_timestamp;

  drop table fight.game;

  drop type game_state;

  drop extension if exists "pgcrypto";

COMMIT;
