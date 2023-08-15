-- Revert fight:game_player from pg

BEGIN;

  drop trigger update_game_player_timestamp on fight.game_player;
  drop function fight.update_game_player_timestamp;
  drop table fight.game_player;

COMMIT;
