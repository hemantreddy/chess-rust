mod pawn_tests;
mod board_tests;

use super::{Color, Movable, Piece, PieceType};
use crate::board::ChessBoard;

#[test]
fn test_knight_valid_move() {
    let knight = Piece::new(PieceType::Knight, Color::White);
    assert!(knight.is_valid_move((3, 3), (5, 4)));
}

#[test]
fn test_knight_invalid_move() {
    let knight = Piece::new(PieceType::Knight, Color::White);
    assert!(!knight.is_valid_move((3, 3), (4, 4)));
}

#[test]
fn test_pawn_valid_move() {
    let pawn = Piece::new(PieceType::Pawn, Color::White);
    // (valid first double move in the new orientation)
    assert!(pawn.is_valid_move((1, 0), (3, 0)));
}


#[test]
fn test_pawn_invalid_move() {
    let pawn = Piece::new(PieceType::Pawn, Color::White);
    assert!(!pawn.is_valid_move((6, 0), (4, 0)));
}

#[test]
fn test_valid_pawn_first_double_move() {
    let mut board = ChessBoard::new();
    assert!(board.make_move((6, 0), (4, 0))); // Valid first double move
}

#[test]
fn test_invalid_pawn_second_double_move() {
    let mut board = ChessBoard::new();
    assert!(board.make_move((6, 0), (5, 0))); // First single move valid
    assert!(!board.make_move((5, 0), (3, 0))); // Invalid second double move
}

#[test]
fn test_valid_pawn_single_move_after_first_move() {
    let mut board = ChessBoard::new();
    assert!(board.make_move((6, 0), (5, 0))); // First single move valid
    assert!(board.make_move((5, 0), (4, 0))); // Valid single move forward
}
