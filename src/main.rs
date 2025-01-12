use crate::board::ChessBoard;

mod board;
mod pieces;

fn main() {
    let mut board = ChessBoard::new();
    println!("Chess game initialized.");
    println!("White starts first.");
    board.display();

    if board.make_move((6, 0), (5, 0)) {
        println!("Move successful")
    } else {
        println!("Invalid move")
    }
}
