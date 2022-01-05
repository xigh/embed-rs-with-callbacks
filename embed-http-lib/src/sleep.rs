use std::{thread, time};

#[allow(dead_code)]
pub fn sleep_ms(ms: u64) {
    println!("sleep_ms: waiting {}ms", ms);
    let dur = time::Duration::from_millis(ms);
    let now = time::Instant::now();
    thread::sleep(dur);
    println!("sleep_ms: elapsed={:?}", now.elapsed());
}
