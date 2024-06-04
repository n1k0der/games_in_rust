use rand::Rng;

// Game
struct Game {
    board: Board,
    players: (Player, Player),
    on_game: bool,
}

// Game board
struct GameBoard {
    size: (i32, i32),
    players_positions: 
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
    board: GameBoard,

}


impl Player {
    fn movement(&mut self) {
        
    }
}

fn main() {

}