-- Revert fight:games from pg

BEGIN;

  drop trigger update_game_timestamp on fight.game;
  drop function fight.update_game_timestamp;

  drop table fight.game;

  drop type fight."GameState";

COMMIT;
