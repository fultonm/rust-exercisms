pub fn main() {
    let len = 10;
    let mut zero_vec = vec![0; len];
    for i in 0..len {
        for j in (0..len).filter(|x| x % i == 0) {
            zero_vec[i] = j;
        }
    }
    
}
