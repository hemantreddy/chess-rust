use crate::board::Board;

mod board;
mod pieces;

fn main() {
    let board = Board::new();
    board.display()
}
