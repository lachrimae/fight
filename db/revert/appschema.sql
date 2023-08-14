-- Revert fight:appschema from pg

BEGIN;

  drop schema fight;

COMMIT;
