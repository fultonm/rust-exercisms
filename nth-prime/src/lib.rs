extern crate itertools;
use itertools::Itertools;

pub fn nth(n: usize) -> Result<usize, &'static str> {
    if n < 1 {
        return Err("Need to take at least the 1st prime");
    }
    let size: usize = (2 as f64 * n as f64 * (n as f64).ln()).ceil() as usize + 15; // Set an upper bound for seiving.
    println!("Size is {0}", size);
    let size_sqrt: usize = (size as f64).sqrt().ceil() as usize;
    let mut nums: Vec<usize> = vec![0, size];
    println!("Nums size is {0}", nums.len());
    let primes: Vec<usize> = sieve(&mut nums, size, size_sqrt);
    println!("Elements in primes: {0}", primes.len());
    return Ok(*primes.last().unwrap());
}

fn sieve(nums: &mut [usize], size: usize, size_sqrt: usize) -> Vec<usize> {
    for i in 0..size {
        nums[i] = i;
    }
    for i in 0..size {
        if i < 2 {
            continue;
        } else if nums[i] > size_sqrt {
            break;
        }
        for j in (nums[i].pow(2)..size).step(nums[i]) {
            nums[j] = 0;
        }
    }
    return nums.iter().filter(|&x| *x != 0).map(|&x| x as usize).collect();
}
