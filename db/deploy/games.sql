-- Deploy fight:games to pg
-- requires: appschema

BEGIN;

  create type game_state as enum('Lobbied', 'Started', 'Completed', 'Cancelled');

  create extension if not exists "pgcrypto";

  create table fight.games (
    id uuid primary key default gen_random_uuid(),
    state game_state not null default 'Lobbied'
  );

COMMIT;
