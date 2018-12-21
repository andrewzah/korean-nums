pub enum Sign {
    Plus,
    Minus
}

impl Sign {
    pub fn to_str(&self) -> &str {
        match self {
            &Sign::Plus => "플러스",
            &Sign::Minus => "마이너스"
        }
    }
}

pub enum MathOp {
    Add,
    Divide,
    Multiply,
    Subtract,
}

impl MathOp {
    pub fn from_char(c: &char) -> Option<MathOp> {
        match c {
            '+' => Some(MathOp::Add),
            '/' => Some(MathOp::Divide),
            '*' => Some(MathOp::Multiply),
            '-' => Some(MathOp::Subtract),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &MathOp::Add => "더하기",
            &MathOp::Divide => "나누기",
            &MathOp::Multiply => "곱하기",
            &MathOp::Subtract => "빼기",
        }
    }
}

pub enum MathExpression {
    Pow,
    Fraction,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    Log
}

impl MathExpression {
    pub fn from_char(s: &str) -> Option<MathExpression> {
        match s {
            "^" => Some(MathExpression::Pow),
            "/" => Some(MathExpression::Fraction),
            "<" => Some(MathExpression::LessThan),
            ">" => Some(MathExpression::GreaterThan),
            "=" => Some(MathExpression::Equal),
            "!=" | "<>" | "=/=" => Some(MathExpression::NotEqual),
            "log" => Some(MathExpression::Log),
            _ => None
        }
    }

    pub fn to_str(&self, curr_num: &str, next_num: &str) -> String {
        match self {
            &MathExpression::Pow => format!("{}의 {}승", curr_num, next_num),
            &MathExpression::Fraction => format!("{}분의 {}", curr_num, next_num),
            &MathExpression::LessThan => format!("{}는 {}보다 작다", curr_num, next_num),
            &MathExpression::GreaterThan => format!("{}는 {}보다 크다", curr_num, next_num),
            &MathExpression::Equal => format!("{}는 {}이다", curr_num, next_num),
            &MathExpression::NotEqual => format!("{}는 {}가 아니다", curr_num, next_num),
            &MathExpression::Log => format!("{} 로그 {}", curr_num, next_num),
        }
    }
}
