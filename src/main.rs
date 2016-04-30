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

    #[test]
    fn regression_test() {
        assert!(solve(1000) == 233168);
    }
}

mod problem2 {
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
    
    #[test]
    fn regression_test() {
        assert!(solve(4_000_000) == 4613732);
    }
}

mod problem3 {
    use ::std::collections::HashSet;

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

    #[test]
    fn regression_test() {
        assert!(solve(600851475143) == 6857);
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

    #[test]
    fn regression_test() {
        assert!(solve(100, 999) == 906609);
    }
}

mod problem5 {
    use ::std::ops::RangeInclusive;
    fn divisible_by_all(n: u64, mut r: RangeInclusive<u64>) -> bool {
        return r.all(|x| n % x == 0);
    }


    fn smallest_divisible_by(r: RangeInclusive<u64>) -> u64 {
        let step_size = r.clone().last().unwrap();
        return (step_size..).step_by(step_size).find(|&x| divisible_by_all(x, r.clone())).unwrap();
    }

    #[test]
    fn example_works() {
        assert!(divisible_by_all(2520, 1...10));
        assert!(smallest_divisible_by(1...10) == 2520);
    }

    pub fn solve(r: RangeInclusive<u64>) -> u64 {
        return smallest_divisible_by(r);
    }

    #[test]
    fn regression_test() {
        assert!(solve(1...20) == 232792560);
    }
}

mod problem6 {
    use ::std::ops::RangeInclusive;
    fn sum_of_squares(r: RangeInclusive<u64>) -> u64 {
        return r.map(|x| x*x).sum();
    }

    fn square_of_sum(r: RangeInclusive<u64>) -> u64 {
        let x: u64 = r.sum();
        return x*x;
    }

    pub fn solve(r: RangeInclusive<u64>) -> u64 {
        return square_of_sum(r.clone()) - sum_of_squares(r);
    }

    #[test]
    fn example_works() {
        assert!(sum_of_squares(1...10) == 385);
        assert!(square_of_sum(1...10) == 3025);
        assert!(solve(1...10) == 2640);
    }
    
    #[test]
    fn regression_test() {
        assert!(solve(1...100) == 25164150);
    }
}

mod problem7 {
    use ::std::collections::HashMap;
    struct Primes {
        factors: HashMap<u64,u64>,
        current: u64,
    }

    fn primes() -> Primes {
        return Primes {
            factors: HashMap::new(),
            current: 1,
        }
    }

    impl Iterator for Primes {
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

    pub fn solve(n: usize) -> u64 {
        return primes().nth(n - 1).unwrap();
    }

    #[test]
    fn test_primes() {
        assert!(primes().nth(5) == Some(13));
        assert!(primes().nth(500) == Some(3581));
    }
    
    #[test]
    fn regression_test() {
        assert!(solve(10_001) == 104743);
    }
}

mod problem9 {
    fn find_triplet(sum: u64) -> (u64, u64, u64) {
        for a in 1.. {
            for b in a.. {
                let c = sum - a - b;
                if c <= b {
                    break;
                }
                if a*a + b*b == c*c {
                    return (a, b, c)
                }
            }
        }
        unreachable!();
    }

    pub fn solve(sum: u64) -> u64 {
        let (a, b, c) = find_triplet(sum);
        return a*b*c;
    }

    #[test]
    fn example_works() {
        assert!(find_triplet(12) == (3, 4, 5));
    }
    
    #[test]
    fn regression_test() {
        assert!(solve(1000) == 31875000);
    }
}

fn main() {
    
    let p1 = problem1::solve(1000);
    println!("Problem 1: {}", p1);

    let p2 = problem2::solve(4_000_000);
    println!("Problem 2: {}", p2);

    let p3 = problem3::solve(600851475143);
    println!("Problem 3: {}", p3);

    let p4 = problem4::solve(100, 999);
    println!("Problem 4: {}", p4);
    
    let p5 = problem5::solve(1...20);
    println!("Problem 5: {}", p5);
    
    let p6 = problem6::solve(1...100);
    println!("Problem 6: {}", p6);
    
    let p7 = problem7::solve(10_001);
    println!("Problem 7: {}", p7);
    
    let p9 = problem9::solve(1000);
    println!("Problem 9: {}", p9);
}
