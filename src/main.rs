fn main() {
    let board_size: i32 = std::env::args()
        .nth(1).expect("Enter board size.")
        .parse().expect("Board size must be an int.");
    println!("{}", board_size);
}
