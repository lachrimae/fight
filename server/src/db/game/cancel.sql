update fight.game
set state = 'Cancelled'
where id = $1;
