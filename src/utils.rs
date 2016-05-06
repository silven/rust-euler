struct IterDigits {
    n: u64,
}

impl Iterator for IterDigits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let d = self.n % 10;
            self.n = self.n / 10;
            return Some(d as u8);
        }
        return None;
    }
}

pub fn int_to_digits(x: u64) -> Vec<u8> {
    let mut v: Vec<u8> = IterDigits{n: x}.collect();
    v.reverse();
    return v;
}

#[test]
fn convert_int_to_digits() {
    assert!(int_to_digits(123) == vec![1, 2, 3]);
    assert!(int_to_digits(10) == vec![1, 0]);
    assert!(int_to_digits(0) == vec![]);
}
