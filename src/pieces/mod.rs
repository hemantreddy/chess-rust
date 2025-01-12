#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub color: Color,
    pub piece_type: PieceType,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            has_moved: false,
        }
    }

    pub fn set_moved(&mut self) {
        self.has_moved = true;
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

pub trait Movable {
    fn is_valid_move(&self, start: (usize, usize), end: (usize, usize)) -> bool;
}

impl Movable for Piece {
    fn is_valid_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        match self.piece_type {
            PieceType::Pawn => {
                let row_diff = end_row as isize - start_row as isize;
                let col_diff = end_col as isize - start_col as isize;
                if self.color == Color::White {
                    // Forward moves => row_diff == +1 or +2 if hasn't moved
                    // Diagonal capture => row_diff == +1, col_diff.abs() == 1
                    if col_diff == 0 {
                        if !self.has_moved {
                            row_diff == 1 || row_diff == 2
                        } else {
                            row_diff == 1
                        }
                    } else if col_diff.abs() == 1 && row_diff == 1 {
                        true // diagonal capture
                    } else {
                        false
                    }
                } else {
                    // Black => row_diff == -1 or -2; diagonal => row_diff == -1
                    if col_diff == 0 {
                        if !self.has_moved {
                            row_diff == -1 || row_diff == -2
                        } else {
                            row_diff == -1
                        }
                    } else if col_diff.abs() == 1 && row_diff == -1 {
                        true
                    } else {
                        false
                    }
                }
            }
            PieceType::Rook => start_row == end_row || start_col == end_col,
            PieceType::Bishop => {
                (start_row as isize - end_row as isize).abs()
                    == (start_col as isize - end_col as isize).abs()
            }
            PieceType::Knight => {
                let row_diff = (start_row as isize - end_row as isize).abs();
                let col_diff = (start_col as isize - end_col as isize).abs();
                (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
            }
            PieceType::Queen => {
                start_row == end_row
                    || start_col == end_col
                    || (start_row as isize - end_row as isize).abs()
                        == (start_col as isize - end_col as isize).abs()
            }
            PieceType::King => {
                let row_diff = (start_row as isize - end_row as isize).abs();
                let col_diff = (start_col as isize - end_col as isize).abs();
                row_diff <= 1 && col_diff <= 1
            }
        }
    }
}
