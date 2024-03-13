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
        let root = Sieve::num_to_index(number.sqrt());
        for i in 0..=root {
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
    println!("Hello, world!");
    for i in [2, 20, 131, 1_000_001, 10_000_001, 100_000_001, 1_000_000_007, 10_000_000_001] {
        print_is_prim(i);
    }
}
