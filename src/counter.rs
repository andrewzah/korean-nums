pub enum Position {
    Empty,
    Tens,
    Hundreds,
    Thousands
}

pub enum Block {
    Empty,
    Man,
    Eok,
    Jo,
    Kyeong
}

impl Position {
    pub fn from_usize(u: usize) -> Option<Position> {
        match u {
            0 => Some(Position::Empty),
            1 => Some(Position::Tens),
            2 => Some(Position::Hundreds),
            3 => Some(Position::Thousands),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            &Position::Empty => "",
            &Position::Tens => "십",
            &Position::Hundreds => "백",
            &Position::Thousands => "천",
        }
    }
}

impl Block {
    pub fn from_index(idx: usize) -> Option<Block> {
        match idx {
            4 => Some(Block::Man),
            8 => Some(Block::Eok),
            12 => Some(Block::Jo),
            16 => Some(Block::Kyeong),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            &Block::Empty => "",
            &Block::Man => "만",
            &Block::Eok => "억",
            &Block::Jo => "조",
            &Block::Kyeong => "경",
        }
    }
}
