pub fn nth(n: usize) -> Result<usize, &'static str> {
    if n < 1 {
        return Err("Need to take at least the 1st prime");
    }
    // Set an upper bound for seiving, ensure size is at least 4 so we can get
    // a square root calculation.
    let size: usize = (2 as f64 * n as f64 * (n as f64).ln()).ceil() as usize + 4;
    let size_sqrt: usize = (size as f64).sqrt().ceil() as usize;
    let mut nums: Vec<usize> = vec![0; size];
    for i in 0..size {
        nums[i] = i;
    }
    let primes: Vec<usize> = sieve(&mut nums, size, size_sqrt);
    return Ok(primes[n - 1]);
}

fn sieve(nums: &mut [usize], size: usize, size_sqrt: usize) -> Vec<usize> {
    for i in 0..size {
        if i < 2 {
            nums[i] = 0;
            continue;
        } else if nums[i] > size_sqrt {
            break;
        }
        for j in (nums[i].pow(2)..size).filter(|x| x % i == 0) {
            nums[j] = 0;
        }
    }
    return nums.iter()
        .filter(|&x| *x != 0)
        .map(|&x| x as usize)
        .collect();
}
