insert into fight.game_player (game_id, user_id)
select $1, $2
where (select count(*) from fight.game_player gp where gp.game_id = $1) < 2
returning id, game_id, user_id, created_at, modified_at
