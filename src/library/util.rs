use std::{fs, time::Instant};

pub fn console_timer(txt: String) -> Instant {
    println!("{txt}");
    return Instant::now();
}

pub fn console_timer_end(now: Instant) {
    let elapsed = now.elapsed();
    println!("[Elapsed: {elapsed:.2?}]\n");
}

pub fn get_size_metadata(dir: &str) -> std::io::Result<u64> {
    let metadata = byte_to(fs::metadata(dir)?.len(), Conversion::Kb);
    Ok(metadata)
}
fn byte_to(len: u64, conv: Conversion) -> u64 {
    match conv {
        Conversion::Kb => len / 1024,
        Conversion::Mb => len / 1_048_576,
    }
}
enum Conversion {
    Mb,
    Kb,
}
