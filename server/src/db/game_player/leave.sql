delete from fight.game_player gp
where gp.game_id = $1 and gp.user_id = $2
