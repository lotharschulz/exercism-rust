#[derive(Debug, PartialEq)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let same_row = self.position.rank == other.position.rank;
        let same_column = self.position.file == other.position.file;
        let same_diagonal = (self.position.rank - other.position.rank).abs()
            == (self.position.file - other.position.file).abs();
        same_row || same_column || same_diagonal
    }
}
