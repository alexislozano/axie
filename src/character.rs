pub enum Character {
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

impl Character {
    pub fn from_luma(luma: u8) -> Self {
        if luma < 25 {
            Self::Zero
        } else if luma < 50 {
            Self::One
        } else if luma < 75 {
            Self::Two
        } else if luma < 100 {
            Self::Three
        } else if luma < 125 {
            Self::Four
        } else if luma < 150 {
            Self::Five
        } else if luma < 175 {
            Self::Six
        } else if luma < 200 {
            Self::Seven
        } else if luma < 225 {
            Self::Eight
        } else {
            Self::Nine
        }
    }

    pub fn ascii(&self) -> &str {
        match self {
            Self::Zero => " ",
            Self::One => ".",
            Self::Two => ":",
            Self::Three => "-",
            Self::Four => "=",
            Self::Five => "+",
            Self::Six => "*",
            Self::Seven => "#",
            Self::Eight => "%",
            Self:: Nine => "@",
        }
    }
}