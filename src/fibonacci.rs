
pub struct Generator<T> {
    curr: T,
    next: T,
}

impl<T: ::std::ops::Add<T, Output=T> + Clone> Iterator for Generator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.clone() + self.next.clone();

        self.curr = self.next.clone();
        self.next = new_next;

        Some(self.curr.clone())
    }
}

pub fn generate<T: ::num::One>() -> Generator<T> {
    Generator { curr: T::one(), next: T::one() }
}

#[test]
fn generator_works() {
    let fibs: Vec<u64> = generate::<u64>().take(10).collect();
    assert!(fibs == vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}
