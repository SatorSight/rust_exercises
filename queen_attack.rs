#[derive(Debug)]
pub struct ChessPosition{
    x: i32,
    y: i32
}

#[derive(Debug)]
pub struct Queen{
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { x: file, y:rank })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let w = &self.position;
        let b = &other.position;

        if (w.x - b.x).abs() == (w.y - b.y).abs() {
            return true;
        }

        if w.x == b.x || w.y == b.y {
            return true;
        }

        false
    }
}