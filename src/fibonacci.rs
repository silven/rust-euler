
pub struct Generator {
    curr: u64,
    next: u64,
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

pub fn generate() -> Generator {
    Generator { curr: 1, next: 1 }
}

#[test]
fn generator_works() {
    let fibs: Vec<u64> = generate().take(10).collect();
    assert!(fibs == vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}
