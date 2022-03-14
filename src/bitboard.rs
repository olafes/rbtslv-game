use core::arch::x86_64::*;

#[repr(C)]
pub union Bitboard {
  arr: [u16; 16],
  m256i: __m256i
}

impl Clone for Bitboard {
    fn clone(&self) -> Self {
      Self {
        arr: unsafe { self.arr }.clone()
      }
    }
}