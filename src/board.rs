use super::bitboard::Bitboard;

#[derive(Clone)]
pub struct Board {
  pub targets: [Bitboard; 4],
  pub blockers: [Bitboard; 4]
}