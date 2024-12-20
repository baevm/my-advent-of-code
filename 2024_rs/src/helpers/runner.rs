use std::time::Instant;

pub fn run_with_timing<F: Fn()>(func: F, part: i8) {
    let now = Instant::now();
    func();
    let elapsed1 = now.elapsed();

    println!("Time for Part {}: {:.2?}\n", part, elapsed1);
}
