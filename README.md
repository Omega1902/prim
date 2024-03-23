# Prime

I just wanna try out some prime stuff and have some fun with it.

## Information on internals

### Using a boolean vector missing even numbers

Pros:

- half the memory - footprint
- only need to go through half of the numbers

Cons:

- Need calculations from index to real number
- Need to sort out even numbers before using the sieve

### Using a boolean Bitvector instead of Vector of numbers for the result primes

Using the Sieve of Eratosthenes I use a [BitVector](https://docs.rs/bitvec/latest/bitvec/) on operating and saving the result. Every even number is also removed, so the first 8 bits represent the odd numbers from 3 to 17.  
But is this the approach with the lowest memory footprint, or should I use a Vector of numbers at some point?

A Vector on u8 could store primes up to 255 and uses 1 byte per prims. So the memory footprint comes down to how many primes are between 3 and 255. Same goes for Vector of u16 for the primes between 257 and 65535 and so on, although each prime number is going to consume 2 bytes instead of just 1. Here is how the prime numbers are distributed on several hights including wether it would be usefull to store them in a Vector of numbers instead:

Distribution from 3 until to 255 (1 Byte area):  
21.03 % -> Better stored in sieve (threshold: 6.25 %)

Distribution from 257 until to 65.535 (2 Byte area):  
9.94 % -> Better stored in sieve (threshold: 3.12 %)

Distribution from 65.537 until to 999.999 (4 Byte area):  
7.70 % -> Better stored in sieve (threshold: 1.56 %)

Distribution from 1.000.001 until to 9.999.999 (4 Byte area):  
6.51 % -> Better stored in sieve (threshold: 1.56 %)

Distribution from 10.000.001 until to 99.999.999 (4 Byte area):  
5.66 % -> Better stored in sieve (threshold: 1.56 %)

Distribution from 100.000.001 until to 999.999.999 (4 Byte area):  
5.01 % -> Better stored in sieve (threshold: 1.56 %)

Distribution from 1.000.000.001 until to 4.294.967.295 (4 Byte area):  
4.63 % -> Better stored in sieve (threshold: 1.56 %)

Distribution from 4.294.967.297 until to 9.999.999.999 (8 Byte area):  
4.41 % -> Better stored in sieve (threshold: 0.78 %)

Distribution from 10.000.000.001 until to 100.000.000.001 (8 Byte area):  
4.07 % -> Better stored in sieve (threshold: 0.78 %)

(Percentage is primes / numbers, threshold is 1 divded by 16 times count of bytes)

So it seems like so far the prime numbers are better of beeing saved inside of the sieve

