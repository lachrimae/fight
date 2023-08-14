-- Revert fight:games from pg

BEGIN;

  drop table fight.games;

  drop type game_state;

COMMIT;
