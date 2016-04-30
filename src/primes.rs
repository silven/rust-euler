use std::collections::HashMap;

pub struct Generator {
    factors: HashMap<u64, u64>,
    current: u64,
}

pub fn generate() -> Generator {
    return Generator {
        factors: HashMap::new(),
        current: 1,
    };
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        for x in self.current + 1.. {
            match self.factors.remove(&x) {
                None => {
                    self.factors.insert(x * x, x);
                    self.current = x;
                    return Some(self.current);
                }
                Some(f) => {
                    let non_prime = (x + f..).step_by(f).find(|v| !self.factors.contains_key(v));
                    self.factors.insert(non_prime.unwrap(), f);
                }
            }
        }
        return None;
    }
}

#[test]
fn generator_works() {
    let small_primes: Vec<u64> = generate().take(4).collect();
    assert!(small_primes == vec![2, 3, 5, 7])
}

#[test]
fn test_specific_primes() {
    assert!(generate().nth(5) == Some(13));
    assert!(generate().nth(500) == Some(3581));
}
