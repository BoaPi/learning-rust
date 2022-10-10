type BoardPiece = [isize; 3];
type BoardLine = [BoardPiece; 3];

pub type Position = (isize, isize);

pub enum PieceType {
    EmptyPiece = -1,
    BigRing = 0,
    MedRing = 1,
    SmlRing = 2,
}

pub enum BoardState {
    Playing,
    CatsGame,
    Winner(u8),
}

pub struct Board {
    pub board: [BoardLine; 3],
    pub state: BoardState,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            board: [[[-1; 3]; 3]; 3],
            state: BoardState::Playing,
        };
    }

    // (A)
    // get position values
    // get current piece type of the board in given position
    //
    // (B)
    // if current piece type is empty return false to indicate position on 
    // board is already taken
    pub fn submit(&mut self, player: u8, pos: Position, piece_type: PieceType) -> bool {
        // (A)
        let (x, y) = pos;
        let piece = self.board[y as usize][x as usize];

        // (B)
        if piece != PieceType::EmptyPiece {
            return false;
        }
    }
}
