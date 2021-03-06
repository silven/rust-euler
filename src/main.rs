#![feature(step_by, step_trait)]
#![feature(inclusive_range, inclusive_range_syntax)]
#![feature(unboxed_closures, fn_traits)]

extern crate num;
extern crate itertools;

mod primes;
mod fibonacci;

#[macro_use]
mod utils;

use utils::time;

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
}

mod problem2 {
    pub fn solve(max_val: u64) -> u64 {
        let fibs = ::fibonacci::generate::<u64>().filter(|&n| n % 2 == 0).take_while(|&n| n < max_val);
        return fibs.sum();
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
}

mod problem4 {
    fn is_palimdrome_number(n: u64) -> bool {
        let digits = ::utils::int_to_digits(n);
        let mut reversed = digits.clone();
        reversed.reverse();
        return digits == reversed;
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
}

mod problem7 {
    pub fn solve(n: usize) -> u64 {
        return ::primes::generate().nth(n - 1).unwrap();
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
}

mod problem12 {
    use ::std::iter::once;

    fn count_divisors(n: u64) -> usize {
        return (1...n / 2).filter(|x| n % x == 0).chain(once(n)).count();
    }

    // Triangle number at index i is == i*(i+1)/2
    // Since i and i+1 is co-prime, we can compute
    // the number of divisors as count_divisors(i)
    // x count_divisors(i+1). Since only one of
    // i and i+1 is even, we adjust for the
    // factor 1 / 2 accordingly.
    //
    fn triangles_divisors(i: u64) -> usize {
        if i % 2 == 0 {
            count_divisors(i / 2) * count_divisors(i + 1)
        } else {
            count_divisors(i) * count_divisors((i + 1) / 2)
        }
    }

    pub fn solve(n: usize) -> u64 {
        let index = (1..).find(|&i| triangles_divisors(i) > n).unwrap();
        return index * (index + 1) / 2;
    }

    #[test]
    fn test_count_divisors() {
        assert!(count_divisors(10) == 4);
        assert!(count_divisors(28) == 6);
    }

    #[test]
    fn example_works() {
        assert!(solve(5) == 28);
    }
}


mod problem14 {
    use std::ops::RangeInclusive;
    struct Collatz {
        n: Option<u64>,
    }

    fn collatz_from(n: u64) -> Collatz {
        return Collatz { n: Some(n) };
    }

    impl Iterator for Collatz {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.n;
            let next = match current {
                None => None,
                Some(1) => None,
                Some(x) if x % 2 == 0 => Some(x / 2),
                Some(x) => Some(x * 3 + 1),
            };
            self.n = next;
            return current;
        }
    }

    fn collatz_length(n: u64) -> usize {
        return collatz_from(n).count();
    }

    pub fn solve(r: RangeInclusive<u64>) -> u64 {
        return r.max_by_key(|&n| collatz_length(n)).unwrap();
    }

    #[test]
    fn example_sequence() {
        let seq: Vec<u64> = collatz_from(13).collect();
        assert!(seq == vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_solver() {
        assert!(solve(1...10) == 9);
    }
}

mod problem15 {
    fn choose(n: u64, k: u64) -> u64 {
        return (1...k).fold(1, |acc, i| acc * (n + 1 - i) / i);
    }

    pub fn solve(size: u64) -> u64 {
        return choose(2 * size, size);
    }

    #[test]
    fn test_choose() {
        assert!(choose(5, 2) == 10);
    }

    #[test]
    fn example() {
        assert!(solve(2) == 6);
    }
}

mod problem16 {
    // Multiplies an array of digits by a factor.
    // Returns a new vector with the digits in the
    // result. For simplicity's sake, the least
    // significant digit is located at index zero,
    // making the array look reversed, but it still works.
    //
    fn mul(digits: &[u64], factor: u64) -> Vec<u64> {
        let mut next_digits = vec![];
        let mut carry = 0;
        for d in digits {
            let value = factor * d + carry;
            carry = value / 10;
            next_digits.push(value % 10);
        }
        if carry > 0 {
            next_digits.push(carry);
        }
        return next_digits;
    }

    pub fn solve(factor: u64, power: u64) -> u64 {
        let mut digits = vec![1];
        for _ in 1...power {
            digits = mul(&digits, factor);
        }
        return digits.iter().sum();
    }

    #[test]
    fn test_mul() {
        assert!(mul(&[2, 3], 2) == vec![4, 6]);
    }

    #[test]
    fn test_mul_internal_carry() {
        assert!(mul(&[5, 1], 2) == vec![0, 3]);
    }

    #[test]
    fn test_mul_external_carry() {
        assert!(mul(&[8], 8) == vec![4, 6]);
        assert!(mul(&[0, 1], 10) == vec![0, 0, 1]);
    }

    #[test]
    fn example() {
        assert!(solve(2, 15) == 26);
    }
}

mod problem18 {
    use ::std::cmp::max;

    pub fn input() -> Vec<Vec<u64>> {
        return vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 04, 82, 47, 65],
            vec![19, 01, 23, 75, 03, 34],
            vec![88, 02, 77, 73, 07, 63, 67],
            vec![99, 65, 04, 28, 06, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
        ];
    }

    fn route_sum(data: &Vec<Vec<u64>>, row: usize, index: usize) -> u64 {
        if row >= data.len() {
            return 0;
        }

        let value = data[row][index];
        return value +
               max(route_sum(data, row + 1, index),
                   route_sum(data, row + 1, index + 1));
    }

    pub fn solve(data: &Vec<Vec<u64>>) -> u64 {
        return route_sum(data, 0, 0);
    }

    #[test]
    fn example() {
        let example_data = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];

        assert!(solve(&example_data) == 23);
    }
}

mod problem20 {
    use ::num::bigint::{BigUint, ToBigUint};
    use ::num::traits::{Zero, One};
    use ::num::integer::Integer;
    use ::num::ToPrimitive;

    struct Digitizer {
        n: Option<BigUint>,
    }

    impl Iterator for Digitizer {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let ten: BigUint = 10.to_biguint().unwrap();

            let mut current = None;
            let mut next = None;

            if let Some(ref n) = self.n {
                let (quotient, remainder) = n.div_rem(&ten);
                next = Some(quotient);
                current = remainder.to_u64();
            }

            if let Some(n) = next {
                if n > Zero::zero() {
                    self.n = Some(n);
                } else {
                    self.n = None;
                }
            }

            return current;
        }
    }

    fn digits_in(x: BigUint) -> Vec<u64> {
        return Digitizer { n: Some(x) }.collect();
    }

    fn big_factorial(x: u64) -> BigUint {
        return (2...x).fold(One::one(), |acc, n| acc * n.to_biguint().unwrap());
    }

    pub fn solve(n: u64) -> u64 {
        return digits_in(big_factorial(n)).iter().sum();
    }

    #[test]
    fn test_digits_in() {
        let a: BigUint = big_factorial(10);
        let ds = digits_in(a);
        assert!(ds == vec![0, 0, 8, 8, 2, 6, 3]);
    }

    #[test]
    fn test_factorial() {
        let a: BigUint = big_factorial(10);
        assert!(a == 3628800.to_biguint().unwrap());
    }

    #[test]
    fn example() {
        assert!(solve(10) == 27);
    }
}

mod problem21 {
    use ::std::collections::HashSet;

    fn divisors_sum(n: usize) -> usize {
        return (1...n / 2).filter(|x| n % x == 0).sum();
    }

    pub fn solve(limit: usize) -> usize {
        let mut div_sum = ::utils::memoize(divisors_sum);

        let mut amicable = HashSet::new();
        for a in 1..limit {
            let b = div_sum.call(a);
            if a != b && div_sum.call(b) == a {
                amicable.insert(a);
                amicable.insert(b);
            }
        }
        return amicable.iter().sum();
    }

    #[test]
    fn example() {
        assert!(divisors_sum(220) == 284);
        assert!(divisors_sum(284) == 220);
    }
}


mod problem22 {
    use ::std::fs::File;
    use ::std::io::Read;

    fn score(name: &str) -> usize {
        // The byte for A is 65 in ascii, so C - 64 should be 3
        return name.as_bytes().iter().fold(0, |acc, b| acc + (b - 64) as usize);
    }

    #[test]
    fn test_name_score() {
        assert!(score("COLIN") == 53);
    }

    pub fn read_names() -> Vec<String> {
        let mut f = File::open("data/p022_names.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        return s.split(",").map(|s| s.to_owned()).collect();
    }

    pub fn solve() -> usize {
        let the_names = read_names();
        let mut names: Vec<&str> = the_names.iter().map(|s| s.trim_matches('"')).collect();
        names.sort();
        return names.iter().enumerate().fold(0, |acc, (idx, name)| acc + (idx + 1) * score(name));
    }
}

mod problem23 {
    use ::std::collections::HashSet;
    use itertools::Itertools;

    fn divisors_sum(n: usize) -> usize {
        return (1...n / 2).filter(|x| n % x == 0).sum();
    }

    fn is_abundant(n: usize) -> bool {
        return divisors_sum(n) > n;
    }

    pub fn solve() -> usize {
        let limit = 28123;

        let abundant_numbers: Vec<usize> = (12...limit).filter(|&n| is_abundant(n)).collect();
        let possible_combinations = abundant_numbers.iter().cartesian_product(abundant_numbers.iter());
        let possible_sums: HashSet<usize> = possible_combinations.map(|(a, b)| a + b).collect();
        let non_summable_sum = (1...limit).filter(|x| !possible_sums.contains(&x)).sum();

        return non_summable_sum;
    }

    #[test]
    fn example() {
        let x: Vec<(usize, char)> = (0..2).cartesian_product("αβ".chars()).collect();

        assert!(x == vec![(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β')]);
        assert!(is_abundant(12) == true);
    }
}

mod problem24 {
    use itertools::Itertools;

    fn state<T: Copy + ::std::fmt::Debug>(xs: &[T]) -> Vec<Vec<T>> {
        // 1 2 3
        // 1 3 2
        // 2 1 3
        // 2 3 1
        // 3 1 2
        // 3 2 1
        if xs.len() == 2 {
            return vec![
                vec![xs[0], xs[1]],
                vec![xs[1], xs[0]],
            ]
        }

        let mut result: Vec<Vec<T>> = Vec::new();
        for (idx, elem) in xs.iter().enumerate() {
            let rest = {
                let mut tmp = xs.to_vec();
                tmp.remove(idx);
                tmp
            };

            for mut permutation in state(&rest) {
                permutation.insert(0, *elem);
                result.push(permutation);
            }
        }
        return result;
    }

    fn lexical_perm(digits: &[usize]) -> Vec<String> {
        return state(digits).iter().map(|xs| xs.into_iter().join("")).collect::<Vec<String>>();
    }

    pub fn solve(nth: usize) -> String {
        return lexical_perm(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).iter().nth(nth-1).unwrap().clone();
    }

    #[test]
    fn example() {
        assert!(state(&vec![2, 3]) == vec![
            vec![2, 3],
            vec![3, 2],
        ]);

        assert!(state(&vec![1, 2, 3]) == vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);

        let answer = vec!["012", "021", "102", "120", "201", "210"];
        let my_answer = lexical_perm(&vec![0, 1, 2]);
        println!("{:?} != {:?}", my_answer, answer);
        assert!(answer == my_answer);

    }
}

mod problem25 {
    use ::num::bigint::{BigUint, ToBigUint};

    fn index_for_fib_of_at_least(min_value: BigUint) -> usize {
        let min = min_value.to_biguint().unwrap();
        let idx_fibs = (2..).zip(::fibonacci::generate::<BigUint>());
        return idx_fibs.skip_while(|&(_, ref n)| n < &min).map(|(idx, _)| idx).next().unwrap();
    }

    pub fn solve(n_digits: usize) -> usize {
        let min_val = ::num::pow(10.to_biguint().unwrap(), n_digits-1);
        return index_for_fib_of_at_least(min_val);
    }

    #[test]
    fn test_example() {
        let fib_idx = index_for_fib_of_at_least(100.to_biguint().unwrap());
        assert_eq!(fib_idx, 12);
    }
}


mod problem27 {
    fn seq_of_primes<F, C>(generator: F, mut check: C) -> usize
        where F: Fn(i64)->i64,
              C: FnMut(i64)->bool,
    {
        return (0..).take_while(|&n| check(generator(n).abs())).count();
    }

    pub fn solve() -> i64 {
        let mut memo_is_prime = ::utils::memoize(::primes::is_prime);
        let mut longest_yet = 0;
        let mut coefs = (0, 0);
        for a in -999...999 {
            for b in -1000...1000 {
                let seq = seq_of_primes(|n| n*n + a*n + b, |x| memo_is_prime.call(x));
                if seq > longest_yet {
                    coefs = (a, b);
                    longest_yet = seq;
                }
            }
        }
        return coefs.0 * coefs.1;
    }

    #[test]
    fn examples() {
        let mut memo_is_prime = ::utils::memoize(::primes::is_prime);
        assert_eq!(seq_of_primes(|n| n*n + n + 41,      |x| memo_is_prime.call(x)), 40);
        assert_eq!(seq_of_primes(|n| n*n - 79*n + 1601, |x| memo_is_prime.call(x)), 80);
    }
}

mod problem28 {
    fn ring_corners(n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        // ring n = 1
        // 7 8 9
        // 6   2
        // 5 4 3
        //
        // x = (2*n+1)^2
        // sum = x + (x - 2n) + (x - 4n) + (x - 6n)
        // => f(n) = 4(2n+1)^2 – 12n
        return 4*(n*2+1).pow(2) - 12*n;
    }

    fn sum_until(ring: usize) -> usize {
        return (0...ring).map(ring_corners).sum();
    }

    pub fn solve(size: usize) -> usize {
        let n_rings = (size-1)/2;
        return sum_until(n_rings);
    }

    #[test]
    fn first_ring() {
        assert_eq!(ring_corners(0), 1);
        assert_eq!(ring_corners(1), 9+7+5+3);
        assert_eq!(sum_until(1), 9+7+5+3 + 1);
    }
}

mod problem29 {
    use itertools::Itertools;
    use std::ops::RangeInclusive;
    use std::collections::HashSet;
    use num::bigint::{BigUint, ToBigUint};

    fn digits(bases: RangeInclusive<u64>, pows: RangeInclusive<u64>) -> HashSet<BigUint> {
        return bases.map(|a|a.to_biguint().unwrap()).cartesian_product(pows).map(|(a, b)|
            ::num::pow(a, b as usize)).collect::<HashSet<BigUint>>();
    }

    pub fn solve(r: RangeInclusive<u64>) -> usize {
        return digits(r.clone(), r).len();
    }

    #[test]
    fn example() {
        let ds = digits(2...5, 2...5);
        assert_eq!(ds.len(), 15);
        //assert_eq!(ds, set![4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125]);

    }
}

mod problem31 {
    fn ways(mut coins: Vec<u32>, target: u32) -> Vec<Vec<(u32, u32)>> {
        let mut result = Vec::new();

        if coins.len() == 1 {
            return vec![
                vec![(target, 1)],
            ];
        }

        let my_coin = coins.pop().unwrap();
        for n in 0...(target / my_coin) {
            if n*my_coin < target {
                let remaining = target - n*my_coin;
                for mut other_coins in ways(coins.clone(), remaining) {
                    if n > 0 {
                        other_coins.insert(0, (n, my_coin));
                    }
                    result.push(other_coins);
                }
            } else if n*my_coin == target {
                result.push(vec![(n, my_coin)]);
            }
        }
        return result;
    }

   pub fn solve() -> usize {
       let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
       return ways(coins, 200).len();
   }

}

mod problem35 {
    use std::collections::HashSet;

    fn digits_to_int(ds: &Vec<u64>) -> u64 {
        let mut result = 0;
        for &d in ds {
            result *= 10;
            result += d;
        }
        return result;
    }

    fn rotations_of(n: u64) -> Vec<u64> {
        let mut digits = ::utils::int_to_digits(n);
        let mut results = vec![digits.clone()];
        for _ in 1..digits.len() {
            let last = digits.pop().unwrap();
            digits.insert(0, last);
            results.push(digits.clone());
        }
        return results.iter().map(digits_to_int).collect::<Vec<u64>>();

    }

    fn is_circular(prime: u64, known_primes: &HashSet<u64>) -> bool {
        return rotations_of(prime).into_iter().all(|rot| known_primes.contains(&rot));
    }

    pub fn solve(limit: u64) -> usize {
        let ps = ::primes::generate().take_while(|&n| n < limit).collect::<HashSet<u64>>();
        let circular = ps.iter().filter(|&p| is_circular(*p, &ps));
        return circular.count();
    }

    #[test]
    fn example_197() {
        let primes = ::primes::generate().take_while(|&n| n < 1000).collect::<HashSet<u64>>();
        assert_eq!(rotations_of(197), vec![197, 719, 971]);
        assert!(is_circular(197, &primes));
    }

}

fn main() {
    time("Problem 1 ", || problem1::solve(1000));

    time("Problem 2 ", || problem2::solve(4_000_000));

    time("Problem 3 ", || problem3::solve(600851475143));

    time("Problem 4 ", || problem4::solve(100, 999));

    time("Problem 5 ", || problem5::solve(1...20));

    time("Problem 6 ", || problem6::solve(1...100));

    time("Problem 7 ", || problem7::solve(10_001));

    time("Problem 9 ", || problem9::solve(1000));

    time("Problem 10", || problem10::solve(2_000_000));

    time("Problem 12", || problem12::solve(500));

    time("Problem 14", || problem14::solve(1...1_000_000));

    time("Problem 15", || problem15::solve(20));

    time("Problem 16", || problem16::solve(2, 1000));

    time("Problem 18", || problem18::solve(&::problem18::input()));

    time("Problem 20", || problem20::solve(100));

    time("Problem 21", || problem21::solve(10000));

    time("Problem 22", || problem22::solve());

    time("Problem 23", || problem23::solve());

    time("Problem 24", || problem24::solve(1_000_000));

    time("Problem 25", || problem25::solve(1000));

    time("Problem 27", || problem27::solve());

    time("Problem 28", || problem28::solve(1001));

    time("Problem 29", || problem29::solve(2...100));

    time("Problem 31 ", || problem31::solve());

    time("Problem 35", || problem35::solve(1_000_000));
}
