-- Verify fight:games on pg

BEGIN;

  select id, state, created_at, modified_at
    from fight.game
  where false;

ROLLBACK;
