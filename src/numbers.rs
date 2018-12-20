#[allow(dead_code)]
pub enum SinoNumber {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl PartialEq<i32> for SinoNumber {
    fn eq(&self, other: &i32) -> bool {
        self.to_i32().eq(other)
    }
}

impl SinoNumber {
    pub fn from_char(c: &char) -> Option<SinoNumber> {
        match c {
            '0' => Some(SinoNumber::Zero),
            '1' => Some(SinoNumber::One),
            '2' => Some(SinoNumber::Two),
            '3' => Some(SinoNumber::Three),
            '4' => Some(SinoNumber::Four),
            '5' => Some(SinoNumber::Five),
            '6' => Some(SinoNumber::Six),
            '7' => Some(SinoNumber::Seven),
            '8' => Some(SinoNumber::Eight),
            '9' => Some(SinoNumber::Nine),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn to_i32(&self) -> i32 {
        match self {
            &SinoNumber::Zero => 0,
            &SinoNumber::One => 1,
            &SinoNumber::Two => 2,
            &SinoNumber::Three => 3,
            &SinoNumber::Four => 4,
            &SinoNumber::Five => 5,
            &SinoNumber::Six => 6,
            &SinoNumber::Seven => 7,
            &SinoNumber::Eight => 8,
            &SinoNumber::Nine => 9,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &SinoNumber::Zero => "공",
            &SinoNumber::One => "일",
            &SinoNumber::Two => "이",
            &SinoNumber::Three => "삼",
            &SinoNumber::Four => "사",
            &SinoNumber::Five => "오",
            &SinoNumber::Six => "육",
            &SinoNumber::Seven => "칠",
            &SinoNumber::Eight => "팔",
            &SinoNumber::Nine => "구",
        }
    }

    #[allow(dead_code)]
    pub fn to_hanja(&self) -> &str {
        match self {
            &SinoNumber::Zero => "零/〇",
            &SinoNumber::One => "一",
            &SinoNumber::Two => "二",
            &SinoNumber::Three => "三",
            &SinoNumber::Four => "四",
            &SinoNumber::Five => "五",
            &SinoNumber::Six => "六",
            &SinoNumber::Seven => "七",
            &SinoNumber::Eight => "八",
            &SinoNumber::Nine => "九",
            //&SinoNumber::Ten => "十",
        }
    }

}

#[allow(dead_code)]
pub enum KoreanNumber {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
}

impl KoreanNumber {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<KoreanNumber> {
        match s {
            "0" => Some(KoreanNumber::Zero),
            "1" => Some(KoreanNumber::One),
            "2" => Some(KoreanNumber::Two),
            "3" => Some(KoreanNumber::Three),
            "4" => Some(KoreanNumber::Four),
            "5" => Some(KoreanNumber::Five),
            "6" => Some(KoreanNumber::Six),
            "7" => Some(KoreanNumber::Seven),
            "8" => Some(KoreanNumber::Eight),
            "9" => Some(KoreanNumber::Nine),
            "10" => Some(KoreanNumber::Ten),
            "20" => Some(KoreanNumber::Twenty),
            "30" => Some(KoreanNumber::Thirty),
            "40" => Some(KoreanNumber::Forty),
            "50" => Some(KoreanNumber::Fifty),
            "60" => Some(KoreanNumber::Sixty),
            "70" => Some(KoreanNumber::Seventy),
            "80" => Some(KoreanNumber::Eighty),
            "90" => Some(KoreanNumber::Ninety),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &str {
        match self {
            &KoreanNumber::Zero => "영/제로",
            &KoreanNumber::One => "하나",
            &KoreanNumber::Two => "둘",
            &KoreanNumber::Three => "셋",
            &KoreanNumber::Four => "넷",
            &KoreanNumber::Five => "다섯",
            &KoreanNumber::Six => "여섯",
            &KoreanNumber::Seven => "일곱",
            &KoreanNumber::Eight => "여덟",
            &KoreanNumber::Nine => "아홉",
            &KoreanNumber::Ten => "열",
            &KoreanNumber::Twenty => "스무",
            &KoreanNumber::Thirty => "서른",
            &KoreanNumber::Forty => "마흔",
            &KoreanNumber::Fifty => "쉰",
            &KoreanNumber::Sixty => "예순",
            &KoreanNumber::Seventy => "일흔",
            &KoreanNumber::Eighty => "여든",
            &KoreanNumber::Ninety => "아흔",
        }
    }

    #[allow(dead_code)]
    pub fn to_i32(&self) -> i32 {
        match self {
            &KoreanNumber::Zero => 0,
            &KoreanNumber::One => 1,
            &KoreanNumber::Two => 2,
            &KoreanNumber::Three => 3,
            &KoreanNumber::Four => 4,
            &KoreanNumber::Five => 5,
            &KoreanNumber::Six => 6,
            &KoreanNumber::Seven => 7,
            &KoreanNumber::Eight => 8,
            &KoreanNumber::Nine => 9,
            &KoreanNumber::Ten => 20,
            &KoreanNumber::Twenty => 20,
            &KoreanNumber::Thirty => 30,
            &KoreanNumber::Forty => 40,
            &KoreanNumber::Fifty => 50,
            &KoreanNumber::Sixty => 60,
            &KoreanNumber::Seventy => 70,
            &KoreanNumber::Eighty => 80,
            &KoreanNumber::Ninety => 80,
        }
    }
}
