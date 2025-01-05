use super::{Piece, PieceType, Color, Movable};

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
    assert!(pawn.is_valid_move((6, 0), (5, 0)));
}

#[test]
fn test_pawn_invalid_move() {
    let pawn = Piece::new(PieceType::Pawn, Color::White);
    assert!(pawn.is_valid_move((6, 0), (4, 0)));
}