use std::fmt;

// Numbers that have their own specific names.
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

#[allow(dead_code)]
pub enum KoreanNumber {
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

impl fmt::Display for SinoNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_i32())
    }
}

impl fmt::Display for PureNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_i32())
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        self.to_i32().eq(other)
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

    #[allow(dead_code)]
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
            &Number::Ten => 20,
            &Number::Twenty => 20,
            &Number::Thirty => 30,
            &Number::Forty => 40,
            &Number::Fifty => 50,
            &Number::Sixty => 60,
            &Number::Seventy => 70,
            &Number::Eighty => 80,
            &Number::Ninety => 80,
        }
    }

    pub fn to_str_sino(&self) -> &str {
        match self {
            &Number::Zero => "공",
            &Number::One => "일",
            &Number::Two => "이",
            &Number::Three => "삼",
            &Number::Four => "사",
            &Number::Five => "오",
            &Number::Six => "육",
            &Number::Seven => "칠",
            &Number::Eight => "팔",
            &Number::Nine => "구",
            _ => ""
        }
    }

    pub fn to_hanja(&self) -> 

}


