insert into fight.game (state) values ('Lobbied')
returning id, state, created_at, modified_at;
