pub enum Block {
    Man,
    Eok,
    Jo,
    Kyeong,
    Hae,
    Ja,
    Yang,
    Gu,
    Gan,
    Jeong,
    Jae,
    Geug,
}

impl Block {
    pub fn from_usize(idx: usize) -> Option<Block> {
        match idx {
            4 => Some(Block::Man),
            8 => Some(Block::Eok),
            12 => Some(Block::Jo),
            16 => Some(Block::Kyeong),
            20 => Some(Block::Hae),
            24 => Some(Block::Ja),
            28 => Some(Block::Yang),
            32 => Some(Block::Gu),
            36 => Some(Block::Gan),
            40 => Some(Block::Jeong),
            44 => Some(Block::Jae),
            48 => Some(Block::Geug),
            _ => None,
        }
    }

    pub fn to_str_with_space(&self) -> String {
        format!("{}{}", " ", self.to_str())
    }

    pub fn to_str(&self) -> &str {
        match self {
            &Block::Man => "만",
            &Block::Eok => "억",
            &Block::Jo => "조",
            &Block::Kyeong => "경",
            &Block::Hae => "해",
            &Block::Ja => "자",
            &Block::Yang => "양",
            &Block::Gu => "구",
            &Block::Gan => "간",
            &Block::Jeong => "정",
            &Block::Jae => "재",
            &Block::Geug => "극",
        }
    }

    #[allow(dead_code)]
    pub fn to_hanja(&self) -> &str {
        match self {
            &Block::Man => "萬",
            &Block::Eok => "億",
            &Block::Jo => "兆",
            &Block::Kyeong => "京",
            &Block::Hae => "垓",
            &Block::Ja => "秭",
            &Block::Yang => "穰",
            &Block::Gu => "溝",
            &Block::Gan => "澗",
            &Block::Jeong => "正",
            &Block::Jae => "載",
            &Block::Geug => "極",
        }
    }
}
