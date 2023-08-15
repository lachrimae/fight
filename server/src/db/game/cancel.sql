update fight.game
set state = 'Cancelled'
where id = $1
returning id, state, created_at, modified_at;
