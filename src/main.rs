#![feature(iter_arith)]
#![feature(step_by)]
#![feature(inclusive_range, inclusive_range_syntax)]

mod primes;
mod fibonacci;

mod problem1 {
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
    pub fn solve(max_val: u64) -> u64 {
        let fibs = ::fibonacci::generate().filter(|&n| n % 2 == 0).take_while(|&n| n < max_val);
        return fibs.sum();
    }

    #[test]
    fn regression_test() {
        assert!(solve(4_000_000) == 4613732);
    }
}

mod problem3 {
    fn factors_for(n: u64) -> Vec<u64> {
        let limit = (n as f64).sqrt().ceil() as u64;
        return ::primes::generate().take_while(|&x| x < limit).filter(|x| n % x == 0).collect();
    }

    pub fn solve(n: u64) -> u64 {
        return factors_for(n).pop().unwrap();
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
        for a in min..max {
            for b in a..max {
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
    use std::ops::RangeInclusive;
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
    use std::ops::RangeInclusive;
    fn sum_of_squares(r: RangeInclusive<u64>) -> u64 {
        return r.map(|x| x * x).sum();
    }

    fn square_of_sum(r: RangeInclusive<u64>) -> u64 {
        let x: u64 = r.sum();
        return x * x;
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
    pub fn solve(n: usize) -> u64 {
        return ::primes::generate().nth(n - 1).unwrap();
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
                if a * a + b * b == c * c {
                    return (a, b, c);
                }
            }
        }
        unreachable!();
    }

    pub fn solve(sum: u64) -> u64 {
        let (a, b, c) = find_triplet(sum);
        return a * b * c;
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

mod problem10 {
    fn sum_primes_less_than(limit: u64) -> u64 {
        return ::primes::generate().take_while(|&n| n < limit).sum();
    }

    pub fn solve(limit: u64) -> u64 {
        return sum_primes_less_than(limit);
    }

    #[test]
    fn example_works() {
        assert!(sum_primes_less_than(10) == 17);
    }
    
    #[test]
    fn regression_test() {
        assert!(solve(2_000_000) == 142913828922);
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

    let p10 = problem10::solve(2_000_000);
    println!("Problem 10: {}", p10);
}
