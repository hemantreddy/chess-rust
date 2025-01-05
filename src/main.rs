use crate::board::Board;

mod board;
mod pieces;

fn main() {
    let board = Board::new();
    board.display();

    match board.get_piece_at(0, 0) {
        Ok(Some(piece)) => println!("Found piece: {:?}", piece),
        Ok(None) => println!("No piece found at this position."),
        Err(e) => println!("Error: {}", e),
    }
}
