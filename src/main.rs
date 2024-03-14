use std::io;

use ::num::integer::Roots;
use bitvec::prelude::*;

trait PrimCalc {
    fn is_prim(num: usize) -> bool;
}

struct Sieve {}

impl Sieve {
    fn num_to_index(number: usize) -> usize {
        (number - 1) / 2 - 1
    }

    fn index_to_num(index: usize) -> usize {
        (index + 1) * 2 + 1
    }
}

impl PrimCalc for Sieve {
    fn is_prim(number: usize) -> bool {
        if number == 2 {
            return true;
        }
        if number < 2 || number % 2 == 0 {
            return false;
        }
        let number_index = Sieve::num_to_index(number);
        let mut sieve = bitvec![1; number_index + 1];
        let mut root = number.sqrt();
        if root % 2 == 0 {
            root -= 1;
        }
        if root < 3 {
            root = 3;
        }
        let root_index = Sieve::num_to_index(root);
        for i in 0..=root_index {
            if !sieve[i] {
                continue;
            }
            let cur_prim = Sieve::index_to_num(i);
            for mul in (i + cur_prim..=number_index).step_by(cur_prim) {
                sieve.set(mul, false);
            }
        }

        sieve[number_index]
    }
}

fn print_is_prim(number: usize) {
    println!("{number} is a prim: {}", Sieve::is_prim(number));
}

fn main() {
    loop {
        println!("Type in the next variable to check if it is a prim");
        let mut input = String::new();
        let input_length = io::stdin().read_line(&mut input);
        if input_length.is_err() {
            println!("Error reading line");
            continue;
        }
        match input.trim().parse::<usize>() {
            Ok(input_number) => print_is_prim(input_number),
            Err(_) => println!("Not possible to convert '{}' into a positive integer", input.trim()),
        }
    }
    // for i in [2, 20, 131, 1_000_001, 10_000_001, 100_000_001, 1_000_000_007, 10_000_000_001] {
    //     print_is_prim(i);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_index_calc() {
        assert_eq!(Sieve::num_to_index(3), 0);
        assert_eq!(Sieve::num_to_index(5), 1);
        assert_eq!(Sieve::index_to_num(0), 3);
        assert_eq!(Sieve::index_to_num(1), 5);
    }

    #[test]
    fn test_sieve() {
        assert_eq!(Sieve::is_prim(2), true);
        assert_eq!(Sieve::is_prim(3), true);
        assert_eq!(Sieve::is_prim(4), false);
        assert_eq!(Sieve::is_prim(25), false);
        assert_eq!(Sieve::is_prim(131), true);
        assert_eq!(Sieve::is_prim(1_000_001), false);
        assert_eq!(Sieve::is_prim(1_000_003), true);
    }
}
