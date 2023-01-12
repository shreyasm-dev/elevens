use rand::{thread_rng, seq::SliceRandom};
use crate::card::{Card, NumberedCard, FaceCard, Play};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
      }
    }

    plays
  }

  pub fn is_game_won(self) -> bool {
    self.deck.cards.is_empty()
  }

  pub fn is_game_lost(self) -> bool {
    self.get_valid_plays().is_empty()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deck {
  pub cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Self {
    let mut cards = Vec::new();

    for _ in 1..=4 {
      for number in 1..=10 {
        cards.push(Card::Number(NumberedCard::from(number)));
      }
  
      for face in [FaceCard::Jack, FaceCard::Queen, FaceCard::King].iter() {
        cards.push(Card::Face(*face));
      }
    }

    Self { cards }
  }

  pub fn shuffle(&mut self) {
    self.cards.shuffle(&mut thread_rng());
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
  pub cards: Vec<Card>,
}

impl Board {
  pub fn new() -> Self {
    Self { cards: Vec::new() }
  }

  pub fn add_card(&mut self, card: Card) {
    self.cards.push(card);
  }

  pub fn play_cards(&mut self, cards: Vec<Card>) {
    for card in cards {
      self.cards.remove(self.cards.iter().position(|c| *c == card).unwrap());
    }
  }

  pub fn fill(&mut self, deck: &mut Deck) {
    while self.cards.len() < 9 {
      self.add_card(deck.cards.pop().unwrap());
    }
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

impl AsRef<Board>for Board {
  fn as_ref(&self) -> &Board {
    &self
  }
}
