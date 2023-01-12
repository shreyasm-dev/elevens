#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Card {
  Number(NumberedCard),
  Face(FaceCard),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberedCard {
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
}

impl NumberedCard {
  pub fn from(number: u8) -> Self {
    match number {
      1 => Self::One,
      2 => Self::Two,
      3 => Self::Three,
      4 => Self::Four,
      5 => Self::Five,
      6 => Self::Six,
      7 => Self::Seven,
      8 => Self::Eight,
      9 => Self::Nine,
      10 => Self::Ten,
      _ => panic!("NumberedCard::from() called with invalid number"),
    }
  }

  pub fn get_complement(self) -> NumberedCard {
    match self {
      NumberedCard::One => Self::Ten,
      NumberedCard::Two => Self::Nine,
      NumberedCard::Three => Self::Eight,
      NumberedCard::Four => Self::Seven,
      NumberedCard::Five => Self::Six,
      NumberedCard::Six => Self::Five,
      NumberedCard::Seven => Self::Four,
      NumberedCard::Eight => Self::Three,
      NumberedCard::Nine => Self::Two,
      NumberedCard::Ten => Self::One,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceCard {
  Jack,
  Queen,
  King,
}

impl FaceCard {
  pub fn get_others(self) -> (FaceCard, FaceCard) {
    match self {
      FaceCard::Jack => (Self::Queen, Self::King),
      FaceCard::Queen => (Self::Jack, Self::King),
      FaceCard::King => (Self::Jack, Self::Queen),
    }
  }
}
