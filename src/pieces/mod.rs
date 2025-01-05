#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn symbol(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::King, Color::White) => '♔',
            (PieceType::Queen, Color::White) => '♕',
            (PieceType::Rook, Color::White) => '♖',
            (PieceType::Bishop, Color::White) => '♗',
            (PieceType::Knight, Color::White) => '♘',
            (PieceType::Pawn, Color::White) => '♙',
            (PieceType::King, Color::Black) => '♚',
            (PieceType::Queen, Color::Black) => '♛',
            (PieceType::Rook, Color::Black) => '♜',
            (PieceType::Bishop, Color::Black) => '♝',
            (PieceType::Knight, Color::Black) => '♞',
            (PieceType::Pawn, Color::Black) => '♟',
        }
    }
}
