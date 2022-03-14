use crate::{direction::{self, Direction}, Board, Agent, utils};
use super::bitboard::Bitboard;

use core::arch::x86_64::*;

#[derive(Clone)]
pub struct State {
  pub agents: [Bitboard; 4],
}
impl State {
  pub fn r#move(&mut self, agent: Agent, direction: Direction, board: &Board) {
    let mut blockers = board.blockers[direction as usize].clone();
    match direction {
      Direction::Top => {
        let tmp = blockers.arr[8];
        blockers.m256i = unsafe { _mm256_slli_si256(blockers, 1) };
        blockers.arr[7] = tmp;

        let hashed = unsafe { }

        let target = unsafe { _mm256_and_si256(blockers.m256i, utils::TOP_REACH[block]) };
      },
      Direction::Right => todo!(),
      Direction::Bottom => todo!(),
      Direction::Left => todo!(),
    }
  }
}