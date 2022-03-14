use super::board::Board;
use super::state::State;
use super::r#move::Move;

pub struct Game {
  board: Board,
  state: State,
}
impl Game {
  pub fn new(board: Board, state: State) -> Self {
    Self {
      board,
      state,
    }
  }
  pub fn get_board(&self) -> &Board {
    &self.board
  }
  pub fn get_state(&self) -> &State {
    &self.state
  }
  pub fn make_move(&mut self, r#move: &Move) {
   unimplemented!() 
  }
}