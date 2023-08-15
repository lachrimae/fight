-- Verify fight:game_player on pg

BEGIN;

  select id, game_id, user_id
    from fight.game_player
  where false;

ROLLBACK;
