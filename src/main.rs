use ::num::integer::Roots;

trait PrimCalc {
    fn is_prim(num: usize) -> bool;
}

struct Sieve {}

impl PrimCalc for Sieve {
    fn is_prim(number: usize) -> bool {
        if number < 2 {
            return false;
        }
        let mut sieve = vec![true; number + 1];
        let root = number.sqrt();
        for i in 2..=root {
            if !sieve[i] {
                continue;
            }
            for mul in (i * 2..=number).step_by(i) {
                sieve[mul] = false;
            }
        }

        sieve[number]
    }
}

fn print_is_prim(number: usize) {
    println!("{number} is a prim: {}", Sieve::is_prim(number));
}

fn main() {
    println!("Hello, world!");
    for i in [
        2,
        20,
        131,
        1_000_001,
        10_000_001,
        100_000_001,
        1_000_000_001,
        10_000_000_001,
    ] {
        print_is_prim(i);
    }
}
