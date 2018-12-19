pub enum Block {
    Man,
    Eok,
    Jo,
    Kyeong
}

impl Block {
    pub fn from_usize(idx: usize) -> Option<Block> {
        match idx {
            4 => Some(Block::Man),
            8 => Some(Block::Eok),
            12 => Some(Block::Jo),
            16 => Some(Block::Kyeong),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &Block::Man => " 만",
            &Block::Eok => " 억",
            &Block::Jo => " 조",
            &Block::Kyeong => " 경",
        }
    }
}
