use rand::Rng

// Game
struct Game {
    board: Board,
    players: (Player, Player),
    on_game: bool,
}

// Game board
struct Board {
    size: (i32, i32),
}

// Player

enum PlayerMovements {
    Quiet,
    Attack,
    Prowl,
}


struct Player{
    name: String,
    position: (i32, i32),
    live_player_close: bool,
    player_detected: bool, 

}


impl Player {
    fn movement(&mut self) {
        
    }
}

fn main() {

}