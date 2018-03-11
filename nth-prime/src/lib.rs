use std::result::Result;

/**
 * Michael Fulton
 * 03/05/2018
 *
 * Given a number n, determine what the nth prime is.
 *
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
 * the 6th prime is 13.
 *
 */

pub fn nth(n: usize) -> Result<usize, &'static str> {
    if n < 1 {
        return Err("Cannot find a prime which comes before the first prime number. (n < 1)");
    }
    Ok(prime_at_position(n))
}

/**
 * Finds the nth prime number by calculating the maximum possible value of such a prime
 * and building an array of all the primes up to that value.
 */
fn prime_at_position(n: usize) -> usize {
    let size: usize = ((2 * n) as f64 * (n as f64).ln()).ceil() as usize + 4;
    let size_sqrt: usize = (size as f64).sqrt().ceil() as usize;
    let mut nums: Vec<usize> = vec![0; size];
    //
    for i in 2..size {
        nums[i] = i;
    }
    let primes: Vec<usize> = eratosthenes_sieve(&mut nums, size, size_sqrt);
    primes[n - 1]
}

/**
 * Rules out the numbers which cannot be prime.
 */
fn eratosthenes_sieve(nums: &mut [usize], size: usize, size_sqrt: usize) -> Vec<usize> {
    for i in 2..size {
        if nums[i] == 0 {
            continue;
        } else if nums[i] > size_sqrt {
            break;
        }
        for j in (nums[i].pow(2)..size).filter(|x| x % i == 0) {
            nums[j] = 0;
        }
    }
    nums.iter()
        .filter(|&x| *x != 0)
        .map(|&x| x as usize)
        .collect()
}
