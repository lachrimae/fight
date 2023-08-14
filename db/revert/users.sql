-- Revert fight:users from pg

BEGIN;

  drop trigger update_user_timestamp on fight.user;
  drop function update_user_timestamp;
  drop table fight.user;

COMMIT;
