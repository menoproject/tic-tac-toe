struct Board {
    size: i32
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "super test")
    }
}

fn main() {
    let size: i32 = std::env::args()
        .nth(1).expect("Enter board size.")
        .parse().expect("Board size must be an int.");
    
    let board = Board { size };

    for _ in 0..1 {
        let mut player_move = String::new();
        std::io::stdin().read_line(&mut player_move).expect("Please enter a move");

        println!("{}", player_move);
    }

    println!("{}: {}", board, board.size);
}
