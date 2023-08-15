update fight.game g
set state = 'Cancelled'
where g.id = $1
and (select count(*) from fight.game_player gp where gp.game_id = $1) = 0
