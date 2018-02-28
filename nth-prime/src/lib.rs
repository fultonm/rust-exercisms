#![feature(iterator_step_by)]
pub fn nth(n: usize) {
    let size: usize = (2 as f64 * n as f64 * (n as f64).ln()) as usize; // Set an upper bound for seiving.
    let size_sqrt: usize = (size as f64).sqrt().ceil() as usize;
    let mut nums: Vec<&mut usize> = Vec::new();
    sieve(nums, &size, &size_sqrt);
}

fn sieve(nums: [&mut usize], size: &usize, size_sqrt: &usize) {
    for i in 0..*size {
        *nums[i] = i;
    }
    for num in nums {
        if **num < 2 {
            continue;
        } else if **num > *size_sqrt {
            break;
        }
        for x in (num.pow(2)..*size).step_by(**num) {
            nums[x] = &mut 0;
        }
    }
}
