#![feature(iter_arith)]
#![feature(step_by)]
#![feature(inclusive_range, inclusive_range_syntax)]

mod problem1 {
    // Find multiples of three and five and sum them
    fn numbers(max: u64) -> Box<Iterator<Item = u64>> {
        let numbers = 1..max;
        return Box::new(numbers.filter(|x| x % 3 == 0 || x % 5 == 0));
    }

    pub fn solve(max: u64) -> u64 {
        return numbers(max).sum();
    }

    #[test]
    fn example_works() {
        let data: Vec<u64> = numbers(10).collect();
        assert!(data == vec![3, 5, 6, 9])
    }
}

mod problem2 {
    // Find sum of even valued fib numbers

    struct Fibonacci {
        curr: u64,
        next: u64,
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            Some(self.curr)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }

    pub fn solve(max_val: u64) -> u64 {
        let fibs = fibonacci().filter(|&n| n % 2 == 0).take_while(|&n| n < max_val);
        return fibs.sum();
    }

    #[test]
    fn fibonnaci_works() {
        let fibs: Vec<u64> = fibonacci().take(10).collect();
        assert!(fibs == vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }
}

mod problem3 {
    use std::collections::HashSet;

    fn primes(n: u64) -> Vec<u64> {
        let mut v = vec![];
        let mut black_list = HashSet::new();

        for x in 2..n {
            if !black_list.contains(&x) {
                for factor in (x..n).step_by(x) {
                    black_list.insert(factor);
                }
                v.push(x);
            }
        }
        return v;
    }

    fn factors_for(n: u64) -> Vec<u64> {
        let limit = (n as f64).sqrt().ceil() as u64;
        return primes(limit).into_iter().filter(|x| n % x == 0).collect();
    }

    pub fn solve(n: u64) -> u64 {
        return factors_for(n).into_iter().last().unwrap();
    }

    #[test]
    fn generator_works() {
        let small_primes = primes(10);
        assert!(small_primes == vec![2, 3, 5, 7])
    }

    #[test]
    fn example_works() {
        let prime_factors = factors_for(13195);
        assert!(prime_factors == vec![5, 7, 13, 29])
    }
}

mod problem4 {
    fn is_palimdrome_number(n: u64) -> bool {
        let byte_array = n.to_string().into_bytes();
        let mut reversed_copy = byte_array.clone();
        reversed_copy.reverse();
        return byte_array == reversed_copy;
    }

    #[test]
    fn test_is_palimdrome() {
        assert!(is_palimdrome_number(1001));
        assert!(is_palimdrome_number(10501));
    }

    fn palimdrome_factors(min: u64, max: u64) -> Vec<(u64, u64)> {
        let mut v = Vec::new();
        for a in min...max {
            for b in a...max {
                let result = a * b;
                if is_palimdrome_number(result) {
                    v.push((a, b));
                }
            }
        }
        return v;
    }

    pub fn solve(min: u64, max: u64) -> u64 {
        let factors = palimdrome_factors(min, max);
        let results = factors.into_iter().map(|(a, b)| a * b);
        return results.max().unwrap();
    }

    #[test]
    fn test_factors() {
        assert!(palimdrome_factors(90, 100) == vec![(91, 99)]);
    }

}

mod problem5 {
    use ::std::ops::RangeInclusive;
    fn divisible_by_all(n: u64, mut r: RangeInclusive<u64>) -> bool {
        return r.all(|x| n % x == 0);
    }


    fn smallest_divisible_by(r: RangeInclusive<u64>) -> u64 {
        return (1..).find(|&x| divisible_by_all(x, r.clone())).unwrap();
    }

    #[test]
    fn example_works() {
        assert!(divisible_by_all(2520, 1...10));
        assert!(smallest_divisible_by(1...10) == 2520);
    }

    pub fn solve(r: RangeInclusive<u64>) -> u64 {
        return smallest_divisible_by(r);
    }

}

fn main() {
    let p1 = problem1::solve(1000);
    println!("Problem 1: {}", p1);

    let p2 = problem2::solve(4000000);
    println!("Problem 2: {}", p2);

    let p3 = problem3::solve(600851475143);
    println!("Problem 3: {}", p3);

    let p4 = problem4::solve(100, 999);
    println!("Problem 4: {}", p4);
    
    let p5 = problem5::solve(1...20);
    println!("Problem 5: {}", p5);
}
