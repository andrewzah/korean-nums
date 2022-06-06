pub enum Place {
    Tens,
    Hundreds,
    Thousands,
}

impl Place {
    pub fn from_usize(i: usize) -> Option<Place> {
        match i {
            1 => Some(Place::Tens),
            2 => Some(Place::Hundreds),
            3 => Some(Place::Thousands),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            &Place::Tens => "십",
            &Place::Hundreds => "백",
            &Place::Thousands => "천",
        }
    }
}
