-- Deploy fight:appschema to pg

BEGIN;

  create extension "pgcrypto";
  create schema fight;

COMMIT;
