use serde::{Deserialize, Serialize};

#[derive(Serialize)]
enum GameState {
    Lobbied,
    Started,
    Completed,
    Cancelled,
}

#[derive(Serialize)]
struct Game {
    id: String,
    state: GameState,
}

#[derive(Serialize)]
struct GameJoinInfo {
    mac_key: String,
    game: Game,
}

#[tokio::main]
async fn main() {}
