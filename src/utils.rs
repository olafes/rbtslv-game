use core::arch::x86_64::*;

use crate::Bitboard;

pub const TOP_REACH: [Bitboard; 256] = [

];
pub const RIGHT_REACH: [Bitboard; 256] = [
  
];
pub const BOTTOM_REACH: [Bitboard; 256] = [
  
];
pub const LEFT_REACH: [Bitboard; 256] = [
  
];

// 0x00 0x00 0x01 0x01 0x00
// 0xFF 0xFF 0x00 0x00 0xFF
// 0b11001
// 0b00110

// val must not equal 0
pub fn unchecked_tzcnt(val: Bitboard) -> (u32, u32) {
  let global = unsafe {
    let tmp = _mm256_cmpeq_epi8(val.m256i, _mm256_setzero_si256());
    let tmp = !_mm256_movemask_epi8(tmp);
    _mm_tzcnt_32(tmp as u32)
  } as u32;

  let local = unsafe {
    let tmp = val.arr[global as usize];
    _mm_tzcnt_32(tmp as u32)
  } as u32;

  (global, local)
}