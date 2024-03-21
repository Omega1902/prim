use std::usize;

use ::num::integer::Roots;
use bitvec::prelude::*;
use num::{Integer, ToPrimitive};
use num_format::{Locale, ToFormattedString};

pub trait PrimCalc {
    /// returns true, if the given number is a prim
    fn is_prim(&mut self, num: u128) -> Option<bool>;

    /// returns the previous prim before the given number, if there is any
    /// previous_prim(3) -> Some(2)
    /// previous_prim(2) -> None
    fn previous_prim(&mut self, num: u128) -> Option<u128>;

    /// returns the next prim after the given number, if the algorithm can calculate it
    fn next_prim(&mut self, num: u128) -> Option<u128>;
}

/// converts a number to the index.
/// Number should be uneven, but it is even, the result is the same as number - 1
/// **Unsafe:** Number must be greater or equal to 3
fn num_to_index(number: usize) -> usize {
    (number - 1) / 2 - 1
}

/// converts an index to the number
/// **Unsafe:** Might overflow
fn index_to_num(index: usize) -> usize {
    (index + 1) * 2 + 1
}

pub struct SieveOfEratosthenes {
    sieve: BitVec<usize>,
    calculated_until_index: usize,
}

impl SieveOfEratosthenes {
    pub fn new() -> SieveOfEratosthenes {
        SieveOfEratosthenes { sieve: bitvec![1;1], calculated_until_index: 0 }
    }

    fn print_distribution(&self, from: usize, to: usize, bytes: u8) {
        let distribution: f64 =
            self.sieve[num_to_index(from)..=num_to_index(to)].count_ones() as f64 / (to - from) as f64;
        println!(
            "Distribution from {} until to {} ({bytes} Byte area):",
            from.to_formatted_string(&Locale::de),
            to.to_formatted_string(&Locale::de),
        );
        let storage_threshold = 1.0 / (16.0 * bytes as f64);
        let storage = if distribution > storage_threshold { "sieve" } else { "result vector" };
        println!(
            "{:.2} % -> better stored in {} (threshold: {:.2} %)",
            distribution * 100.0,
            storage,
            storage_threshold * 100.0
        );
    }
}

impl PrimCalc for SieveOfEratosthenes {
    fn is_prim(&mut self, number: u128) -> Option<bool> {
        if number == 2 || number == 3 {
            return Some(true);
        }
        if number < 2 || number.is_even() {
            return Some(false);
        }
        let mut root = number.sqrt().to_usize()?;
        if root < 3 {
            root = 3;
        }
        let root_index = num_to_index(root);
        let mut root_root = root.sqrt();
        if root_root < 3 {
            root_root = 3;
        }
        let root_root_index = num_to_index(root_root);
        if root_root_index > self.calculated_until_index || root_index >= self.sieve.len() {
            self.sieve = bitvec![1; root_index + 1];
            for i in 0..=root_root_index {
                if !self.sieve[i] {
                    continue;
                }
                let cur_prim = index_to_num(i);
                for mul in (i + cur_prim..=root_index).step_by(cur_prim) {
                    self.sieve.set(mul, false);
                }
            }
            self.calculated_until_index = root_root_index;
        }

        if (index_to_num(self.sieve.len() - 1) as u128) > number {
            return Some(self.sieve[num_to_index(number as usize)]);
        }

        for i in 0..=root_index {
            if !self.sieve[i] {
                continue;
            }

            let cur_prim = index_to_num(i);
            if number % cur_prim as u128 == 0 {
                return Some(false);
            }
        }

        if (root as u64) > 10_000_000_001 {
            self.print_distribution(3, u8::MAX as usize, 1);
            self.print_distribution(u8::MAX as usize + 2, u16::MAX as usize, 2);
            self.print_distribution(u16::MAX as usize + 2, 999_999, 4);
            self.print_distribution(1_000_001, 9_999_999, 4);
            self.print_distribution(10_000_001, 99_999_999, 4);
            self.print_distribution(100_000_001, 999_999_999, 4);
            self.print_distribution(1_000_000_001, u32::MAX as usize, 4);
            self.print_distribution(u32::MAX as usize + 2, 9_999_999_999, 8);
            self.print_distribution(10_000_000_001, root.try_into().unwrap_or(usize::MAX), 8);
        }

        Some(true)
    }

    fn previous_prim(&mut self, num: u128) -> Option<u128> {
        if num <= 2 {
            return None;
        }
        if num == 3 {
            return Some(2);
        }
        let mut cur = num - 1;
        if cur.is_even() {
            cur -= 1;
        }
        loop {
            if self.is_prim(cur).expect("this is not calculateable") {
                return Some(cur);
            }
            cur -= 2;
        }
    }
    fn next_prim(&mut self, num: u128) -> Option<u128> {
        if num < 2 {
            return Some(2);
        }
        let mut cur = num.checked_add(1)?;
        if cur.is_even() {
            cur += 1; // does not need to be checked, since integer MAX is always odd
        }
        loop {
            if self.is_prim(cur)? {
                return Some(cur);
            }
            cur = cur.checked_add(2)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_index_calc() {
        assert_eq!(num_to_index(3), 0);
        assert_eq!(num_to_index(4), 0);
        assert_eq!(num_to_index(5), 1);
        assert_eq!(num_to_index(6), 1);
        assert_eq!(index_to_num(0), 3);
        assert_eq!(index_to_num(1), 5);
    }

    #[test]
    fn test_sieve() {
        assert_eq!(SieveOfEratosthenes::new().is_prim(0), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(1), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(2), Some(true));
        assert_eq!(SieveOfEratosthenes::new().is_prim(3), Some(true));
        assert_eq!(SieveOfEratosthenes::new().is_prim(4), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(5), Some(true));
        assert_eq!(SieveOfEratosthenes::new().is_prim(6), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(7), Some(true));
        assert_eq!(SieveOfEratosthenes::new().is_prim(8), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(9), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(10), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(11), Some(true));
        assert_eq!(SieveOfEratosthenes::new().is_prim(12), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(25), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(131), Some(true));
        assert_eq!(SieveOfEratosthenes::new().is_prim(1_000_001), Some(false));
        assert_eq!(SieveOfEratosthenes::new().is_prim(1_000_003), Some(true));
    }

    #[test]
    fn test_sieve_previous() {
        assert_eq!(SieveOfEratosthenes::new().previous_prim(0), None);
        assert_eq!(SieveOfEratosthenes::new().previous_prim(1), None);
        assert_eq!(SieveOfEratosthenes::new().previous_prim(2), None);
        assert_eq!(SieveOfEratosthenes::new().previous_prim(3), Some(2));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(4), Some(3));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(5), Some(3));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(6), Some(5));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(7), Some(5));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(8), Some(7));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(9), Some(7));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(10), Some(7));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(11), Some(7));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(12), Some(11));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(25), Some(23));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(132), Some(131));
        assert_eq!(SieveOfEratosthenes::new().previous_prim(1_000_004), Some(1_000_003));
    }

    #[test]
    fn test_sieve_next() {
        assert_eq!(SieveOfEratosthenes::new().next_prim(0), Some(2));
        assert_eq!(SieveOfEratosthenes::new().next_prim(1), Some(2));
        assert_eq!(SieveOfEratosthenes::new().next_prim(2), Some(3));
        assert_eq!(SieveOfEratosthenes::new().next_prim(3), Some(5));
        assert_eq!(SieveOfEratosthenes::new().next_prim(4), Some(5));
        assert_eq!(SieveOfEratosthenes::new().next_prim(5), Some(7));
        assert_eq!(SieveOfEratosthenes::new().next_prim(6), Some(7));
        assert_eq!(SieveOfEratosthenes::new().next_prim(7), Some(11));
        assert_eq!(SieveOfEratosthenes::new().next_prim(8), Some(11));
        assert_eq!(SieveOfEratosthenes::new().next_prim(9), Some(11));
        assert_eq!(SieveOfEratosthenes::new().next_prim(10), Some(11));
        assert_eq!(SieveOfEratosthenes::new().next_prim(11), Some(13));
        assert_eq!(SieveOfEratosthenes::new().next_prim(12), Some(13));
        assert_eq!(SieveOfEratosthenes::new().next_prim(25), Some(29));
        assert_eq!(SieveOfEratosthenes::new().next_prim(130), Some(131));
        assert_eq!(SieveOfEratosthenes::new().next_prim(1_000_002), Some(1_000_003));
        assert_eq!(SieveOfEratosthenes::new().next_prim(u128::MAX), None);
    }

    #[test]
    fn test_filled() {
        let mut sieve = SieveOfEratosthenes::new();
        sieve.next_prim(1_000_000);
        let len = sieve.sieve.len();
        assert_eq!(sieve.is_prim(997), Some(true));
        assert_eq!(sieve.is_prim(998), Some(false));
        assert_eq!(sieve.is_prim(999), Some(false));
        assert_eq!(sieve.is_prim(1_000), Some(false));
        assert_eq!(sieve.is_prim(999_983), Some(true));
        assert_eq!(sieve.is_prim(5), Some(true));
        assert_eq!(sieve.sieve.len(), len); // len should not have changed
    }
}
