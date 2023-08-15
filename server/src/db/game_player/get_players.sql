select * from fight.user u
  join fight.game_player gp
    on gp.user_id = u.id
where gp.game_id = $1
