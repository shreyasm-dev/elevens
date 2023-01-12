use crate::card::{Card, FaceCard, NumberedCard, Play};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game {
  pub deck: Deck,
  pub board: Board,
}

impl Game {
  pub fn new() -> Self {
    Self {
      deck: Deck::new(),
      board: Board::new(),
    }
  }

  pub fn init(&mut self) {
    self.deck.shuffle();
    self.board.fill(&mut self.deck);
  }

  pub fn get_valid_plays(self) -> Vec<Play> {
    let mut plays = Vec::new();
    let clone = self.board.cards.clone();

    for card in self.board.cards {
      match card {
        Card::Number(card) => {
          if clone.contains(&Card::Number(card.get_complement())) {
            plays.push(Play::NumberedPair(card));
          }
        }
        Card::Face(card) => {
          let others = card.get_others();

          if clone.contains(&Card::Face(others.0)) && clone.contains(&Card::Face(others.0)) {
            plays.push(Play::FaceTriple);
          }
        }
        Card::Placeholder => panic!("Unexpected error: found placeholder card")
      }
    }

    plays
  }

  pub fn play(&mut self, play: Play) {
    self.board.play(play);
    self.board.fill(&mut self.deck);
  }

  pub fn is_game_won(self) -> bool {
    self.deck.cards.is_empty()
  }

  pub fn is_game_lost(self) -> bool {
    self.get_valid_plays().is_empty()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Deck {
  pub cards: [Card; 52],
}

impl Deck {
  pub fn new() -> Self {
    let mut cards: [Card; 52] = [Card::Placeholder; 52];

    for suit in 1..=4 {
      for number in 1..=10 {
        cards[(suit - 1) * 13 + number - 1] = Card::Number(NumberedCard::from(number));
      }

      for face in 1..=3 {
        cards[(suit - 1) * 13 + 10 + face - 1] = Card::Face(
          [
            FaceCard::Jack,
            FaceCard::Queen,
            FaceCard::King,
          ][face - 1]
        );
      }
    }

    Self { cards }
  }

  pub fn shuffle(&mut self) {
    self.cards.shuffle(&mut thread_rng());
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
  pub cards: [Card; 9],
}

impl Board {
  pub fn new() -> Self {
    Self {
      cards: [Card::Placeholder; 9],
    }
  }

  pub fn play(&mut self, play: Play) {
    let cards = match play {
      Play::NumberedPair(card) => vec![Card::Number(card), Card::Number(card.get_complement())],
      Play::FaceTriple => vec![
        Card::Face(FaceCard::Jack),
        Card::Face(FaceCard::Queen),
        Card::Face(FaceCard::King),
      ],
    };

    for card in cards {
      let index = self.cards.iter().position(|c| *c == card).unwrap();
      self.cards[index] = self.cards[self.cards.len() - 1];
      self.cards[self.cards.len() - 1] = Card::Placeholder;
    }
  }

  pub fn fill(&mut self, deck: &mut Deck) {
    self.cards = self.cards.map(|card| {
      if card == Card::Placeholder {
        let index = deck.cards.iter().position(|c| *c != Card::Placeholder).unwrap();
        let card = deck.cards[index];
        deck.cards[index] = Card::Placeholder;
        card
      } else {
        card
      }
    });
  }
}

// AsRef

impl AsRef<Game> for Game {
  fn as_ref(&self) -> &Game {
    &self
  }
}

impl AsRef<Deck> for Deck {
  fn as_ref(&self) -> &Deck {
    &self
  }
}

impl AsRef<Board> for Board {
  fn as_ref(&self) -> &Board {
    &self
  }
}
