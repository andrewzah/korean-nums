pub enum Sign {
    Plus,
    Minus
}

impl Sign {
    pub fn from_str(s: &str) -> Option<Sign> {
        match s {
            "+" => Some(Sign::Plus),
            "-" => Some(Sign::Minus),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &Sign::Plus => "플러스",
            &Sign::Minus => "마이너스"
        }
    }
}

pub enum KoreanMathOp {
    Add,
    Divide,
    Multiply,
    Subtract,
    Pow,
    Fraction,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    Log
}

impl KoreanMathOp {
    pub fn from_str(s: &str) -> Option<KoreanMathOp> {
        match s {
            "+" => Some(KoreanMathOp::Add),
            "/" => Some(KoreanMathOp::Divide),
            "*" => Some(KoreanMathOp::Multiply),
            "-" => Some(KoreanMathOp::Subtract),
            "^" => Some(KoreanMathOp::Pow),
            "<" => Some(KoreanMathOp::LessThan),
            ">" => Some(KoreanMathOp::GreaterThan),
            "=" => Some(KoreanMathOp::Equal),
            "!=" | "<>" | "=/=" => Some(KoreanMathOp::NotEqual),
            "log" => Some(KoreanMathOp::Log),
            _ => None
        }
    }

    pub fn to_str(&self, left_num: &str, right_num: &str) -> String {
        let unformatted_str = self.unformatted_str();
        let new_str = unformatted_str
            .replace("{1}", left_num)
            .replace("{2}", right_num);

        match self {
            &KoreanMathOp::LessThan |
            &KoreanMathOp::GreaterThan |
            &KoreanMathOp::Equal |
            &KoreanMathOp::NotEqual => {
                return new_str.replace("{3}", "는")
            },
            _ => {
                return new_str
            }
        }
    }

    fn unformatted_str(&self) -> &str {
        match self {
            &KoreanMathOp::Add => "{1} 더하기 {2}",
            &KoreanMathOp::Divide => "{1} 나누기 {2}",
            &KoreanMathOp::Multiply => "{1} 곱하기 {2}",
            &KoreanMathOp::Subtract => "{1} 빼기 {2}",
            &KoreanMathOp::Pow => "{1}의 {2}승",
            &KoreanMathOp::Fraction => "{1}분의 {2}",
            &KoreanMathOp::LessThan => "{1}{3} {2}보다 작다",
            &KoreanMathOp::GreaterThan => "{1}{3} {2}보다 크다",
            &KoreanMathOp::Equal => "{1}{3} {2}이다",
            &KoreanMathOp::NotEqual => "{1}{3} {2}가 아니다",
            &KoreanMathOp::Log => "{1} 로그 {2}",
        }
    }
}
