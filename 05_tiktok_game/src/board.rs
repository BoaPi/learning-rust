type BoardPiece = [isize; 3];
type BoardLine = [BoardPiece; 3];

pub type Position = (isize, isize);

pub enum PieceType {
    EptyPiece = -1,
    BigRing = 0,
    MeRing = 1,
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

    pub fn submit() {}
}