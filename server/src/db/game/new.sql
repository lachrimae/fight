with game as (
 insert into fight.game (state) values ('Lobbied')
   returning id, state, created_at, modified_at
)
insert into fight.game_player (user_id, game_id) values ($1, (select id from game))
returning (select id from game), 
          (select state from game),
          (select created_at from game),
          (select modified_at from game);
