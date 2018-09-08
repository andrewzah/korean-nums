// These are the way Korean groups numbers,
// hence the odd [from an English perspective] names.
pub enum Group {
    Ten,
    Hundred,
    Thousand,
    TenThousand,
    TenTenThousand, // hundred thousand
    HundredTenThousand, // million
    ThousandTenThousand, // 10 million
    HundredMillion,
    TenHundredMillion, // billion
    HundredHundredMillion // 10 billion
}

impl Group {
    // Sino-Korean Numbers (Chinese roots)
    pub fn to_str_sino(&self) -> &str {
        match self {
            &Group::Ten => "십",
            &Group::Hundred => "백",
            &Group::Thousand => "천",
            &Group::TenThousand => "만",
            &Group::TenTenThousand => "십만",
            &Group::HundredTenThousand => "백만",
            &Group::ThousandTenThousand => "천만",
            &Group::HundredMillion => "억",
            &Group::TenHundredMillion => "십억",
            &Group::HundredHundredMillion => "백억",
        }
    }

    pub fn from_length(int: usize) -> Option<Group> {
        match int {
            2 => Some(Group::Ten),
            3 => Some(Group::Hundred),
            4 => Some(Group::Thousand),
            5 => Some(Group::TenThousand),
            6 => Some(Group::TenTenThousand),
            7 => Some(Group::HundredTenThousand),
            8 => Some(Group::ThousandTenThousand),
            9 => Some(Group::HundredMillion),
            10 => Some(Group::TenHundredMillion),
            11 => Some(Group::HundredHundredMillion),
            _ => None
        }
    }

    // Pure Korean numbers
    pub fn to_str_pure(&self) -> &str {
        match self {
            &Group::Ten => "열",
            &Group::Hundred => "",
            &Group::Thousand => "",
            &Group::TenThousand => "",
            &Group::TenTenThousand => "",
            &Group::HundredTenThousand => "",
            &Group::ThousandTenThousand => "",
            &Group::HundredMillion => "",
            &Group::TenHundredMillion => "",
            &Group::HundredHundredMillion => ""
        }
    }
}
