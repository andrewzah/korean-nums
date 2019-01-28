use num::{FromPrimitive, Integer};
use crate::parse;

pub enum NumberSystem{
    PureKorean,
    SinoKorean
}


pub struct KoreanInteger<I>
where I: Copy + ToString + Integer + FromPrimitive {
    pub value: I,
    pub num_system: NumberSystem,
}

impl <I> KoreanInteger<I>
where I: Copy + ToString + Integer + FromPrimitive {
    pub fn from_int(value: I, num_system: NumberSystem) -> KoreanInteger<I> {
        let integer = KoreanInteger {
            value: value,
            num_system,
        };

        integer.validate();
        integer
    }

    pub fn validate(&self) {
        match self.num_system {
            NumberSystem::PureKorean => {
                if self.value > FromPrimitive::from_i8(99).unwrap() {
                    panic!("Pure korean numbers only go up to 99.");
                }
                if self.value < FromPrimitive::from_i8(1).unwrap() {
                    panic!("Input cannot be 0 or negative.")
                }
            },
            NumberSystem::SinoKorean => {},
        }
    }

    pub fn get_hangeul(&self) -> String {
        let prepared_input = self.prepare_input();

        match self.num_system {
            NumberSystem::PureKorean => {
                parse::parse_hangeul_pure(prepared_input)
            },
            NumberSystem::SinoKorean => {
                parse::parse_hangeul_sino(prepared_input)
            }
        }
    }

    fn prepare_input(&self) -> Vec<char>
    {
        let nums = self.value
            .to_string()
            .replace(",", "")
            .chars()
            .rev()
            .collect();

        nums
    }
}

pub enum KoreanNumberSino {
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

pub enum EndingType {
    Vowel,
    Consonant
}

impl EndingType {
    pub fn subject_particle(&self) -> &str {
        match self {
            &EndingType::Vowel => "가",
            &EndingType::Consonant => "이",
        }
    }

    pub fn topic_particle(&self) -> &str {
        match self {
            &EndingType::Vowel => "는",
            &EndingType::Consonant => "은",
        }
    }
}

impl PartialEq<i8> for KoreanNumberSino {
    fn eq(&self, other: &i8) -> bool {
        self.to_i8().eq(other)
    }
}

impl KoreanNumberSino {
    pub fn from_char(c: &char) -> Option<KoreanNumberSino> {
        match c {
            '0' => Some(KoreanNumberSino::Zero),
            '1' => Some(KoreanNumberSino::One),
            '2' => Some(KoreanNumberSino::Two),
            '3' => Some(KoreanNumberSino::Three),
            '4' => Some(KoreanNumberSino::Four),
            '5' => Some(KoreanNumberSino::Five),
            '6' => Some(KoreanNumberSino::Six),
            '7' => Some(KoreanNumberSino::Seven),
            '8' => Some(KoreanNumberSino::Eight),
            '9' => Some(KoreanNumberSino::Nine),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &KoreanNumberSino::Zero => "",
            &KoreanNumberSino::One => "일",
            &KoreanNumberSino::Two => "이",
            &KoreanNumberSino::Three => "삼",
            &KoreanNumberSino::Four => "사",
            &KoreanNumberSino::Five => "오",
            &KoreanNumberSino::Six => "육",
            &KoreanNumberSino::Seven => "칠",
            &KoreanNumberSino::Eight => "팔",
            &KoreanNumberSino::Nine => "구",
        }
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            &KoreanNumberSino::Zero => 0,
            &KoreanNumberSino::One => 1,
            &KoreanNumberSino::Two => 2,
            &KoreanNumberSino::Three => 3,
            &KoreanNumberSino::Four => 4,
            &KoreanNumberSino::Five => 5,
            &KoreanNumberSino::Six => 6,
            &KoreanNumberSino::Seven => 7,
            &KoreanNumberSino::Eight => 8,
            &KoreanNumberSino::Nine => 9,
        }
    }

    pub fn ending_type(&self) -> EndingType {
        match self {
            &KoreanNumberSino::Zero   => EndingType::Consonant,
            &KoreanNumberSino::One    => EndingType::Consonant,
            &KoreanNumberSino::Two    => EndingType::Vowel,
            &KoreanNumberSino::Three  => EndingType::Consonant,
            &KoreanNumberSino::Four   => EndingType::Vowel,
            &KoreanNumberSino::Five   => EndingType::Vowel,
            &KoreanNumberSino::Six    => EndingType::Consonant,
            &KoreanNumberSino::Seven  => EndingType::Consonant,
            &KoreanNumberSino::Eight  => EndingType::Consonant,
            &KoreanNumberSino::Nine   => EndingType::Vowel,
            //&KoreanNumberSino::Ten    => EndingType::Consonant,
        }
    }


    #[allow(dead_code)]
    pub fn to_hanja(&self) -> &str {
        match self {
            &KoreanNumberSino::Zero => "零/〇",
            &KoreanNumberSino::One => "一",
            &KoreanNumberSino::Two => "二",
            &KoreanNumberSino::Three => "三",
            &KoreanNumberSino::Four => "四",
            &KoreanNumberSino::Five => "五",
            &KoreanNumberSino::Six => "六",
            &KoreanNumberSino::Seven => "七",
            &KoreanNumberSino::Eight => "八",
            &KoreanNumberSino::Nine => "九",
            //&KoreanNumberSino::Ten => "十",
        }
    }
}

pub enum KoreanNumberPure {
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

impl KoreanNumberPure {
    pub fn from_char(c: &char) -> Option<KoreanNumberPure> {
        match c {
            '1' => Some(KoreanNumberPure::One),
            '2' => Some(KoreanNumberPure::Two),
            '3' => Some(KoreanNumberPure::Three),
            '4' => Some(KoreanNumberPure::Four),
            '5' => Some(KoreanNumberPure::Five),
            '6' => Some(KoreanNumberPure::Six),
            '7' => Some(KoreanNumberPure::Seven),
            '8' => Some(KoreanNumberPure::Eight),
            '9' => Some(KoreanNumberPure::Nine),
            _ => None
        }
    }

    pub fn from_str(s: &str) -> Option<KoreanNumberPure> {
        match s {
            "10" => Some(KoreanNumberPure::Ten),
            "20" => Some(KoreanNumberPure::Twenty),
            "30" => Some(KoreanNumberPure::Thirty),
            "40" => Some(KoreanNumberPure::Forty),
            "50" => Some(KoreanNumberPure::Fifty),
            "60" => Some(KoreanNumberPure::Sixty),
            "70" => Some(KoreanNumberPure::Seventy),
            "80" => Some(KoreanNumberPure::Eighty),
            "90" => Some(KoreanNumberPure::Ninety),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &KoreanNumberPure::One => "하나",
            &KoreanNumberPure::Two => "둘",
            &KoreanNumberPure::Three => "셋",
            &KoreanNumberPure::Four => "넷",
            &KoreanNumberPure::Five => "다섯",
            &KoreanNumberPure::Six => "여섯",
            &KoreanNumberPure::Seven => "일곱",
            &KoreanNumberPure::Eight => "여덟",
            &KoreanNumberPure::Nine => "아홉",
            &KoreanNumberPure::Ten => "열",
            &KoreanNumberPure::Twenty => "스물",
            &KoreanNumberPure::Thirty => "서른",
            &KoreanNumberPure::Forty => "마흔",
            &KoreanNumberPure::Fifty => "쉰",
            &KoreanNumberPure::Sixty => "예순",
            &KoreanNumberPure::Seventy => "일흔",
            &KoreanNumberPure::Eighty => "여든",
            &KoreanNumberPure::Ninety => "아흔",
        }
    }
}
