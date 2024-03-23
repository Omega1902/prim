use clap::Parser;
use num_format::{Locale, ToFormattedString};

use prime::{BigPrime, PrimeCalc, PrimeCalcExtended};

fn print_is_prime<T: PrimeCalc>(number: u128, prime_solver: &mut T) {
    match prime_solver.is_prime(number) {
        Some(true) => println!("{} is a prime", number.to_formatted_string(&Locale::de)),
        Some(false) => println!("{} is NOT a prime", number.to_formatted_string(&Locale::de)),
        None => println!("Cannot calculate whether {} is a prime", number.to_formatted_string(&Locale::de)),
    }
}

fn print_prev_prime<T: PrimeCalcExtended>(number: u128, prime_solver: &mut T) {
    match prime_solver.previous_prime(number) {
        None => println!("There is no prime before {}", number.to_formatted_string(&Locale::de)),
        Some(prime) => println!(
            "Previous prime before {} is: {}",
            number.to_formatted_string(&Locale::de),
            prime.to_formatted_string(&Locale::de)
        ),
    }
}

fn print_next_prime<T: PrimeCalcExtended>(number: u128, prime_solver: &mut T) {
    match prime_solver.next_prime(number) {
        None => println!("There is no prime calculateable after {}", number.to_formatted_string(&Locale::de)),
        Some(prime) => println!(
            "Next prime after {} is: {}",
            number.to_formatted_string(&Locale::de),
            prime.to_formatted_string(&Locale::de)
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
    /// Number to check if it is a prime. Allows _ as seperator
    #[arg(value_parser = parse_to_integer)]
    number: u128,

    /// Searches for previous prime instead
    #[arg(short, long)]
    previous: bool,

    /// Searches for next prime instead
    #[arg(short, long)]
    next: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut sieve = BigPrime::new();
    if cli.previous {
        print_prev_prime(cli.number, &mut sieve);
    } else if cli.next {
        print_next_prime(cli.number, &mut sieve);
    } else {
        print_is_prime(cli.number, &mut sieve);
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
