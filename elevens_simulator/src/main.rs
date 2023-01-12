mod card;
mod game;

use card::{NumberedCard, Card, FaceCard, Play};
use game::{Game, Board, Deck};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
  let mut n = 1;
  let mut game = Game::new();
  game.init();

  while !simulate_game(&mut game) {
    game = Game::new();
    game.init();
    n += 1;
  }

  println!("Game won in {} iterations", n);
  println!("Verify: {} (deck is empty: {})", game.is_game_won(), game.deck.cards.iter().all(|c| *c == Card::Placeholder));
}

fn simulate_game(game: &mut Game) -> bool {
  if game.is_game_won() || game.is_game_lost() {
    return game.is_game_won();
  }

  let mut plays = game.get_valid_plays();

  if plays.contains(&Play::FaceTriple) {
    game.play(Play::FaceTriple);
  } else {
    game.play(plays.choose(&mut thread_rng()).unwrap().clone());
  }

  simulate_game(game)
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
      Play::FaceTriple,
      Play::FaceTriple,
      Play::FaceTriple
    ]);
  }
}
