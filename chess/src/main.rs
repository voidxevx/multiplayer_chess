pub mod game {
    pub mod board;
}

use crate::game::board::{Board, BoardLocation};

fn main() {
    let mut board: Board = Board::new();
    board.add_layer(String::from("♔"), String::from("♚"));
    board.add_layer(String::from("♕"), String::from("♛"));
    board.add_layer(String::from("♖"), String::from("♜"));
    board.add_layer(String::from("♗"), String::from("♝"));
    board.add_layer(String::from("♘"), String::from("♞"));
    board.add_layer(String::from("♙"), String::from("♟"));

    let loc = BoardLocation::new(1, 2);
    board.toggle_location_active(4, &loc);
    board.set_location_team(&loc, 0);
    let formatted = board.print_board();

    println!("\x1b[H\x1b[2J");
    for i in 0..8 {
        let str: String = formatted[i].clone().into_iter().collect();
        println!("| {} | ", str);
    }

}
