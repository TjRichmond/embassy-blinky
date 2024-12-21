/// Delay function that uses assembly no operation calls
pub fn busy_wait(count: u32) {
  for _ in 0..count {
    cortex_m::asm::nop();
  }
}