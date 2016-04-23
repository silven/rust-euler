#![feature(iter_arith)]
#![feature(step_by)]

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
    use ::std::collections::HashMap;
    
    fn primes(n: u64) -> Vec<u64> {
        let mut v = vec![2];

        let mut black_list = HashMap::new();
        for x in (3..n).step_by(2) {
            if !black_list.contains_key(&x) {
                for factor in (x..n).step_by(x) {
                    black_list.insert(factor, x);
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


fn main() {
    let p1 = problem1::solve(1000);
    println!("Problem 1: {}", p1);

    let p2 = problem2::solve(4000000);
    println!("Problem 2: {}", p2);
    
    let p3 = problem3::solve(600851475143);
    println!("Problem 3: {}", p3);
}
