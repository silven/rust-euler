use std::time::{Duration, Instant};
use std::fmt::Display;

pub fn time<F: FnOnce() -> A, A: Display>(name: &str, func: F) {
    let start = Instant::now();
    let answer = func();
    let duration = start.elapsed();
    println!("{}: {} (took: {})", name, answer, format_duration(duration));
}

pub fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{}s {:.*}ms",
                d.as_secs(),
                3,
                d.subsec_nanos() as f64 / 1_000_000.0)
    } else if d.subsec_nanos() > 1_000_000 {
        format!("{:.*}ms", 3, d.subsec_nanos() as f64 / 1_000_000.0)
    } else if d.subsec_nanos() > 1000 {
        format!("{:.*}μs", 3, d.subsec_nanos() as f64 / 1000.0)
    } else {
        format!("{}ns", d.subsec_nanos())
    }
}

struct IterDigits {
    n: u64,
}

impl Iterator for IterDigits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let d = self.n % 10;
            self.n = self.n / 10;
            return Some(d);
        }
        return None;
    }
}

pub fn int_to_digits(x: u64) -> Vec<u64> {
    let mut v: Vec<u64> = IterDigits { n: x }.collect();
    v.reverse();
    return v;
}

#[test]
fn convert_int_to_digits() {
    assert!(int_to_digits(123) == vec![1, 2, 3]);
    assert!(int_to_digits(10) == vec![1, 0]);
    assert!(int_to_digits(0) == vec![]);
}
