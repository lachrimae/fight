-- Revert fight:appschema from pg

BEGIN;

  drop schema fight;
  drop extension "pgcrypto";

COMMIT;
