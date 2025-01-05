use crate::pieces::{Color, Piece, PieceType};

pub struct Board {
    grid: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[None; 8]; 8],
        };

        // Place pawns
        for i in 0..8 {
            board.grid[1][i] = Some(Piece::new(PieceType::Pawn, Color::Black));
            board.grid[6][i] = Some(Piece::new(PieceType::Pawn, Color::White));
        }

        // Place other pieces
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

        for (i, &piece_type) in back_row.iter().enumerate() {
            board.grid[0][i] = Some(Piece::new(piece_type, Color::Black));
            board.grid[7][i] = Some(Piece::new(piece_type, Color::White));
        }

        board
    }

    pub fn display(&self) {
        println!(" A B C D E F G H");
        for (i, row) in self.grid.iter().enumerate().rev() {
            print!("{} ", i + 1);
            for cell in row.iter() {
                match cell {
                    Some(piece) => print!("{} ", piece.symbol()),
                    None => print!(". "),
                }
            }
            println!("{}", i + 1);
        }
        println!(" A B C D E F G H");
    }
}
