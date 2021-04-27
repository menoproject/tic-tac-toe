use std::io::stdin;

struct Board {
    size: i32
}


fn main() {
    let size: i32 = std::env::args()
        .nth(1).expect("Enter board size.")
        .parse().expect("Board size must be an int.");
    
    let board = Board { size };

    for elem in 0..10 {
        let mut player_move = String::new();
        stdin().read_line(&mut player_move).expect("Please enter a move");

        println!("{}", player_move);
    }

    println!("{}", board.size);
}
