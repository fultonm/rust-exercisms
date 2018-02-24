#![feature(i128_type)]
//#![feature(inclusive_range_syntax)]

pub fn square_of_sum(n: u128) -> u128 {
    //(1...n).fold(0, |sum, x| sum + x).pow(2)
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u128) -> u128 {
    //(1...n).fold(0, |sum, x| sum + x.pow(2))
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u128) -> u128 {
    square_of_sum(n) - sum_of_squares(n)
}