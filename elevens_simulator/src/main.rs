mod card;
mod game;

use card::{Play, Card, NumberedCard, FaceCard};
use game::Game;

fn main() {
  let mut game = Game::new();
  game.init();
  println!("{:#?}", game.board.cards);
  println!("{:#?}", game.get_valid_plays());
}

#[cfg(test)]
mod tests {
  use super::*;

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
