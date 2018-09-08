use std::fmt;

// Numbers that have their own specific names.
pub enum Number {
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
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_i32())
    }
}

impl Number {
    pub fn from_str(s: &str) -> Option<Number> {
        match s {
            "0" => Some(Number::Zero),
            "1" => Some(Number::One),
            "2" => Some(Number::Two),
            "3" => Some(Number::Three),
            "4" => Some(Number::Four),
            "5" => Some(Number::Five),
            "6" => Some(Number::Six),
            "7" => Some(Number::Seven),
            "8" => Some(Number::Eight),
            "9" => Some(Number::Nine),
            "20" => Some(Number::Twenty),
            "30" => Some(Number::Thirty),
            "40" => Some(Number::Forty),
            "50" => Some(Number::Fifty),
            "60" => Some(Number::Sixty),
            _ => None
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            &Number::Zero => 0,
            &Number::One => 1,
            &Number::Two => 2,
            &Number::Three => 3,
            &Number::Four => 4,
            &Number::Five => 5,
            &Number::Six => 6,
            &Number::Seven => 7,
            &Number::Eight => 8,
            &Number::Nine => 9,
            &Number::Twenty => 20,
            &Number::Thirty => 30,
            &Number::Forty => 40,
            &Number::Fifty => 50,
            &Number::Sixty => 60,
        }
    }

    pub fn to_str_sino(&self) -> Option<&str> {
        match self {
            &Number::Zero => Some("<0>"),
            &Number::One => Some("일"),
            &Number::Two => Some("이"),
            &Number::Three => Some("삼"),
            &Number::Four => Some("사"),
            &Number::Five => Some("오"),
            &Number::Six => Some("육"),
            &Number::Seven => Some("칠"),
            &Number::Eight => Some("팔"),
            &Number::Nine => Some("구"),
            _ => None
        }
    }

    pub fn to_str_pure(&self) -> Option<&str> {
        match self {
            &Number::Zero => Some("<0>"),
            &Number::One => Some("하나"),
            &Number::Two => Some("둘"),
            &Number::Three => Some("셋"),
            &Number::Four => Some("넷"),
            &Number::Five => Some("다섯"),
            &Number::Six => Some("여섯"),
            &Number::Seven => Some("일곱"),
            &Number::Eight => Some("여덟"),
            &Number::Nine => Some("아홉"),
            _ => None
        }
    }

    fn to_str_pure_conjugate(&self) -> Option<&str> {
        match self {
            &Number::One => Some("한"),
            &Number::Two => Some("두"),
            &Number::Three => Some("세"),
            &Number::Four => Some("네"),
            _ => None
        }
    }
}

