mod card;
mod game;

use card::{NumberedCard, Card, FaceCard, Play};
use game::{Game, Board, Deck};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
  let mut sum = 0;
  let n = 1000;

  for _ in 0..n {
    sum += simulate_round(|plays, _| plays.choose(&mut thread_rng()).unwrap().clone());
  }

  println!("~{} games played before winning (randomly picked, {} samples)", (sum as f64) / (n as f64), n);
}

fn simulate_round<P: Fn(Vec<Play>, Game) -> Play>(picker: P) -> i64 {
  let mut n = 1;
  let mut game = Game::new();
  game.init();

  while !simulate_game(&mut game, &picker) {
    game = Game::new();
    game.init();
    n += 1;
  }

  assert_eq!(game.is_game_won(), true);
  assert_eq!(game.deck.cards.iter().all(|c| *c == Card::Placeholder), true);

  n
}

fn simulate_game<P: Fn(Vec<Play>, Game) -> Play>(game: &mut Game, picker: P) -> bool {
  if game.is_game_won() || game.is_game_lost() {
    return game.is_game_won();
  }

  game.play(picker(game.get_valid_plays(), *game));

  simulate_game(game, picker)
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
    game.board.cards = [
      Card::Number(NumberedCard::One),
      Card::Number(NumberedCard::Five),
      Card::Number(NumberedCard::Six),
      Card::Number(NumberedCard::Eight),
      Card::Number(NumberedCard::Nine),
      Card::Number(NumberedCard::Ten),
      Card::Face(FaceCard::Jack),
      Card::Face(FaceCard::Queen),
      Card::Face(FaceCard::King),
    ];

    assert_eq!(game.get_valid_plays(), [
      Play::NumberedPair(NumberedCard::One),
      Play::NumberedPair(NumberedCard::Five),
      Play::NumberedPair(NumberedCard::Six),
      Play::NumberedPair(NumberedCard::Ten),
      Play::FaceTriple(FaceCard::Jack),
      Play::FaceTriple(FaceCard::Queen),
      Play::FaceTriple(FaceCard::King),
    ]);
  }
}
