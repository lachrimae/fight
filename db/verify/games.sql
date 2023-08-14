-- Verify fight:games on pg

BEGIN;

  select id, state
    from fight.games
  where false;

ROLLBACK;
