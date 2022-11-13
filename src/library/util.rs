use std::time::Instant;

pub fn console_timer(txt: String) -> Instant {
   println!("{txt}");
   return Instant::now();
}

pub fn console_timer_end(now: Instant) {
   let elapsed = now.elapsed();
   println!("[Elapsed: {elapsed:.2?}]\n");
}