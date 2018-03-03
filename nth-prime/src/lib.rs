pub fn nth(n: usize) -> Result<usize, &'static str> {
    if n < 1 {
        return Err("Need to take at least the 1st prime");
    }
    let size: usize = (2 as f64 * n as f64 * (n as f64).ln()).ceil() as usize + 15; // Set an upper bound for seiving.
    let size_sqrt: usize = (size as f64).sqrt().ceil() as usize;
    let mut nums: Vec<usize> = vec![0; size];
    println!("nums length is {0}", nums.len());
    let primes: Vec<usize> = sieve(&mut nums, size, size_sqrt);
    println!("primes length is {0}", primes.len());
    return Ok(*primes.last().unwrap());
}

fn sieve(nums: &mut [usize], size: usize, size_sqrt: usize) -> Vec<usize> {
    for i in 0..size {
        nums[i] = i;
    }
    for i in 0..size {
        println!("nums[{0}] = {1}", i, nums[i])
    }
    for i in 0..size {
        if i < 2 {
            continue;
        } else if nums[i] > size_sqrt {
            break;
        }
        let step = nums[i];
        println!("This iteration #{0}, nums[i] is {1}", i, nums[i]);
        for j in (nums[i].pow(2)..size).filter(|x| x % step == 0) {
            nums[j] = 0;
        }
    }
    return nums.iter()
        .filter(|&x| *x != 0)
        .map(|&x| x as usize)
        .collect();
}
