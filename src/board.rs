use crate::pieces::{Color, Movable, Piece, PieceType};

pub struct ChessBoard {
    grid: [[Option<Piece>; 8]; 8],
    current_turn: Color,
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[None; 8]; 8],
            current_turn: Color::White,
        };

        // White pawns on row 1
        for i in 0..8 {
            board.grid[1][i] = Some(Piece::new(PieceType::Pawn, Color::White));
        }
        // Black pawns on row 6
        for i in 0..8 {
            board.grid[6][i] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        let back_row = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        // White back row on row 0
        for (i, &piece_type) in back_row.iter().enumerate() {
            board.grid[0][i] = Some(Piece::new(piece_type, Color::White));
        }
        // Black back row on row 7
        for (i, &piece_type) in back_row.iter().enumerate() {
            board.grid[7][i] = Some(Piece::new(piece_type, Color::Black));
        }

        board
    }

    pub fn display(&self) {
        println!("  A B C D E F G H");
        for row_index in (0..8).rev() {
            print!("{} ", row_index + 1);
            for col_index in 0..8 {
                match self.grid[row_index][col_index] {
                    Some(ref piece) => print!("{} ", piece.symbol()),
                    None => print!(". "),
                }
            }
            println!("{}", row_index + 1);
        }
        println!("  A B C D E F G H");
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Option<Piece> {
        if row < 8 && col < 8 {
            self.grid[row][col]
        } else {
            None
        }
    }

    pub fn is_valid_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        if let Some(piece) = self.get_piece(start.0, start.1) {
            if piece.color == self.current_turn {
                return piece.is_valid_move(start, end);
            }
        }
        false
    }

    pub fn make_move(&mut self, start: (usize, usize), end: (usize, usize)) -> bool {
        if let Some(mut piece) = self.grid[start.0][start.1] {
            // Let the piece logic check if the move shape is correct
            if !piece.is_valid_move(start, end) {
                return false;
            }

            let row_diff = end.0 as isize - start.0 as isize;
            let col_diff = end.1 as isize - start.1 as isize;

            if piece.piece_type == PieceType::Pawn {
                match piece.color {
                    Color::White => {
                        // Diagonal capture => row_diff == +1 and col_diff.abs() == 1
                        if col_diff.abs() == 1 && row_diff == 1 {
                            // Must capture an opponent
                            if let Some(dest_piece) = self.grid[end.0][end.1] {
                                if dest_piece.color == piece.color {
                                    return false; // can't capture own piece
                                }
                            } else {
                                return false; // can't move diagonally if no piece to capture
                            }
                        } else {
                            // Forward => col_diff == 0, row_diff is +1 or +2
                            // Already validated by is_valid_move, but we must check occupancy
                            if self.grid[end.0][end.1].is_some() {
                                return false; // can't move onto an occupied square if going straight
                            }
                        }
                    }

                    Color::Black => {
                        // Same idea but row_diff == -1 for diagonal capture
                        if col_diff.abs() == 1 && row_diff == -1 {
                            if let Some(dest_piece) = self.grid[end.0][end.1] {
                                if dest_piece.color == piece.color {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            // Forward => col_diff == 0, row_diff is -1 or -2
                            if self.grid[end.0][end.1].is_some() {
                                return false;
                            }
                        }
                    }
                }
            }

            // Make sure we’re not capturing our own piece if it’s not a pawn scenario
            if let Some(dest_piece) = self.grid[end.0][end.1] {
                if dest_piece.color == piece.color {
                    return false; // can't capture own piece
                }
            }

            // Finally, do the move
            piece.set_moved();
            self.grid[end.0][end.1] = Some(piece);
            self.grid[start.0][start.1] = None;
            self.toggle_turn();
            true
        } else {
            false
        }
    }

    pub fn toggle_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{Color, PieceType};

    #[test]
    fn test_board_initialization() {
        let board = ChessBoard::new();
        assert!(board.get_piece(0, 0).is_some());
        assert!(board.get_piece(7, 7).is_some());
        assert!(board.get_piece(3, 3).is_none());
    }

    #[test]
    fn test_valid_pawn_move() {
        let mut board = ChessBoard::new();
        assert!(board.make_move((6, 0), (5, 0))); // Valid pawn move
    }

    #[test]
    fn test_pawn_valid_first_double_move() {
        let pawn = Piece::new(PieceType::Pawn, Color::White);
        assert!(pawn.is_valid_move((1, 0), (3, 0)));
        assert!(pawn.is_valid_move((1, 0), (2, 0)));
    }


    #[test]
    fn test_toggle_turn() {
        let mut board = ChessBoard::new();
        assert_eq!(board.current_turn, Color::White);
        board.toggle_turn();
        assert_eq!(board.current_turn, Color::Black);
    }

    #[test]
    fn test_invalid_move_to_own_piece() {
        let mut board = ChessBoard::new();
        board.grid[4][4] = Some(Piece::new(PieceType::Pawn, Color::White));
        board.grid[5][4] = Some(Piece::new(PieceType::Pawn, Color::White));

        assert!(!board.make_move((4, 4), (5, 4))); // Invalid move: Own piece at destination
    }

    #[test]
    fn test_valid_pawn_capture() {
        let mut board = ChessBoard::new();
        board.grid[4][4] = Some(Piece::new(PieceType::Pawn, Color::White));
        board.grid[5][5] = Some(Piece::new(PieceType::Pawn, Color::Black));

        assert!(board.make_move((4, 4), (5, 5))); // Valid capture
    }

    #[test]
    fn test_invalid_pawn_straight_capture() {
        let mut board = ChessBoard::new();
        board.grid[4][4] = Some(Piece::new(PieceType::Pawn, Color::White));
        board.grid[5][4] = Some(Piece::new(PieceType::Pawn, Color::Black));

        assert!(!board.make_move((4, 4), (5, 4))); // Invalid: Pawn cannot capture straight
    }
}
