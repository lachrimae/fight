-- Verify fight:appschema on pg

BEGIN;

  select pg_catalog.has_schema_privilege('fight', 'usage');

ROLLBACK;
