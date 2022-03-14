use core::arch::x86_64::*;

#[repr(C)]
pub union Bitboard {
  pub arr: [u16; 16],
  pub m256i: __m256i
}
impl Clone for Bitboard {
  fn clone(&self) -> Self {
    Self {
      arr: unsafe { self.arr }.clone()
    }
  }
}