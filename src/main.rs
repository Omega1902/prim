use clap::Parser;
use num_format::{Locale, ToFormattedString};

use prim::{PrimCalc, SieveOfEratosthenes};

// Distribution from 3 until to 255 (1 Byte area):
// 21.03 % -> Better stored in sieve (threshold: 6.25 %)
// Distribution from 257 until to 65.535 (2 Byte area):
// 9.94 % -> Better stored in sieve (threshold: 3.12 %)
// Distribution from 65.537 until to 999.999 (4 Byte area):
// 7.70 % -> Better stored in sieve (threshold: 1.56 %)
// Distribution from 1.000.001 until to 9.999.999 (4 Byte area):
// 6.51 % -> Better stored in sieve (threshold: 1.56 %)
// Distribution from 10.000.001 until to 99.999.999 (4 Byte area):
// 5.66 % -> Better stored in sieve (threshold: 1.56 %)
// Distribution from 100.000.001 until to 999.999.999 (4 Byte area):
// 5.01 % -> Better stored in sieve (threshold: 1.56 %)
// Distribution from 1.000.000.001 until to 4.294.967.295 (4 Byte area):
// 4.63 % -> Better stored in sieve (threshold: 1.56 %)
// Distribution from 4.294.967.296 until to 9.999.999.999 (8 Byte area):
// 4.41 % -> Better stored in sieve (threshold: 0.78 %)
// Distribution from 10.000.000.001 until to 100.000.000.001 (8 Byte area):
// 4.07 % -> Better stored in sieve (threshold: 0.78 %)

fn print_is_prim<T: PrimCalc>(number: u128, prim_solver: &mut T) {
    match prim_solver.is_prim(number) {
        Some(true) => println!("{} is a prim", number.to_formatted_string(&Locale::de)),
        Some(false) => println!("{} is NOT a prim", number.to_formatted_string(&Locale::de)),
        None => println!("Cannot calculate whether {} is a prim", number.to_formatted_string(&Locale::de)),
    }
}

fn print_prev_prim<T: PrimCalc>(number: u128, prim_solver: &mut T) {
    match prim_solver.previous_prim(number) {
        None => println!("There is no prim before {}", number.to_formatted_string(&Locale::de)),
        Some(prim) => println!(
            "Previous prim before {} is: {}",
            number.to_formatted_string(&Locale::de),
            prim.to_formatted_string(&Locale::de)
        ),
    }
}

fn print_next_prim<T: PrimCalc>(number: u128, prim_solver: &mut T) {
    match prim_solver.next_prim(number) {
        None => println!("There is no prim calculateable after {}", number.to_formatted_string(&Locale::de)),
        Some(prim) => println!(
            "Next prim after {} is: {}",
            number.to_formatted_string(&Locale::de),
            prim.to_formatted_string(&Locale::de)
        ),
    }
}

fn parse_to_integer(input: &str) -> Result<u128, String> {
    match input.replace('_', "").parse::<u128>() {
        Ok(input_number) => Ok(input_number),
        Err(_) => Err(format!("Not possible to convert '{}' into a positive integer", input)),
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number to check if it is a prim
    #[arg(value_parser = parse_to_integer)]
    number: u128,

    /// Searches for previous prim instead
    #[arg(short, long)]
    previous: bool,

    /// Searches for next prim instead
    #[arg(short, long)]
    next: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut sieve = SieveOfEratosthenes::new();
    if cli.previous {
        print_prev_prim(cli.number, &mut sieve);
    } else if cli.next {
        print_next_prim(cli.number, &mut sieve);
    } else {
        print_is_prim(cli.number, &mut sieve);
    }
    // 1_000_001
    // 10_000_001
    // 100_000_001
    // 1_000_000_007
    // 10_000_000_001
    // 100_000_000_003
    // 1_000_000_000_039
    // 10_000_000_000_099
    // 100_000_000_000_097
    // 100_000_000_000_099
    // 1_000_000_000_000_091
    // 10_000_000_000_000_079
    // 100_000_000_000_000_099
    // 1_000_000_000_000_000_003
    // 18_446_744_073_709_551_557
    // 18_446_744_073_709_551_615 // MAX u64
    // 100_000_000_000_000_000_039
    // 1_000_000_000_000_000_000_117
    // 10_000_000_000_000_000_000_009
    // 100_000_000_000_000_000_000_200
}
