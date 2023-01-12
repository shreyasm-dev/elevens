mod card;
mod game;

use card::{NumberedCard, Card, FaceCard};
use game::{Game, Board, Deck};

fn main() {
  let mut game = Game::new();
  game.init();

  println!("{:#?}", game.board);

  let board_clone = game.board.clone();
  let deck_clone = game.deck.clone();

  for card in game.board.cards {
    let clone2 = board_clone.clone();
    let clone3 = deck_clone.clone();
    let clone4 = clone2.clone();

    match card {
      Card::Number(card) => {
        let i_c = i_c(card, clone2);
        let f_c = f_c(card, clone3);
        println!("{:#?} i_c: {}, f_c: {}", card, i_c, f_c);
      }
      Card::Face(card) => {
        let i_f = i_f(card, clone2);
        let f_f = f_f(card, clone3, clone4);
        println!("{:#?} i_f: {}, f_f: {}", card, i_f, f_f);
      }
    }
  }
}

fn i_c(card: NumberedCard, board: Board) -> f64 {
  let complement = card.get_complement();
  let count = board.cards.iter().filter(|c| **c == Card::Number(complement)).count() as i32;

  count as f64
}

fn f_c(card: NumberedCard, deck: Deck) -> f64 {
  let complement = card.get_complement();
  let len = deck.cards.len() as i32;
  let count = deck.cards.iter().filter(|c| **c == Card::Number(complement)).count() as i32;

  (count.pow(2) as f64) / (len as f64)
}

fn i_f(card: FaceCard, board: Board) -> f64 {
  let others = card.get_others();
  let count = board.cards.iter().filter(|c| **c == Card::Face(others.0) || **c == Card::Face(others.1)).count() as i32;

  ((count as f64) / 2.0).powi(2)
}

fn f_f(card: FaceCard, deck: Deck, board: Board) -> f64 {
  let others = card.get_others();
  let count = deck.cards.iter().filter(|c| **c == Card::Face(others.0) || **c == Card::Face(others.1)).count() as i32;
  let i_f = i_f(card, board);

  ((count as f64) / 2.0).powi(2) * i_f
}

#[cfg(test)]
mod tests {
  use crate::card::{Play, Card, NumberedCard, FaceCard};
  use crate::game::Game;

  #[test]
  fn get_valid_plays() {
    let mut game = Game::new();
    game.init();
    game.board.cards = vec![
      Card::Number(NumberedCard::One),
      Card::Number(NumberedCard::Two),
      Card::Number(NumberedCard::Three),
      Card::Number(NumberedCard::Four),
      Card::Number(NumberedCard::Six),
      Card::Number(NumberedCard::Eight),
      Card::Number(NumberedCard::Nine),
      Card::Number(NumberedCard::Ten),
      Card::Face(FaceCard::Jack),
      Card::Face(FaceCard::Queen),
      Card::Face(FaceCard::King),
    ];

    assert_eq!(game.get_valid_plays(), vec![
      Play::NumberedPair(NumberedCard::One),
      Play::NumberedPair(NumberedCard::Two),
      Play::NumberedPair(NumberedCard::Three),
      Play::NumberedPair(NumberedCard::Eight),
      Play::NumberedPair(NumberedCard::Nine),
      Play::NumberedPair(NumberedCard::Ten),
      Play::FaceTriple,
      Play::FaceTriple,
      Play::FaceTriple
    ]);
  }
}
