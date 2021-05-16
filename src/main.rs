struct Board {
    size: i32,
    state: Vec<Vec<i32>>
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0..self.size {
            for _ in 0..self.size {
                write!(f, "| {} |", self.size).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

struct Player {
    id: i32
}

fn main() {
    let size: i32 = std::env::args()
        .nth(1).expect("Enter board size.")
        .parse().expect("Board size must be an int.");

    let state = Vec::new();
    let board = Board { size, state };

    let current_player = 0;
    loop {

        println!("Player {}, please enter a move", current_player);
        let mut player_move = String::new();
        std::io::stdin().read_line(&mut player_move).expect("Please enter a move");
        let player_move_valid = player_move.trim().parse::<i32>();
        match player_move_valid {
            Ok(player_move) => println!("Player move: {}", player_move),
            Err(err) => println!("Error: {}", err)
        }
    }

    println!("{}", board);
}
