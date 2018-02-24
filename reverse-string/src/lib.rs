pub fn reverse(_s: &str) -> String {
    let mut result: String = String::new();
    let len: usize = _s.chars().count();
    for i in 0..len {
        result.push(_s.chars().rev().nth(i).unwrap());
    }
    return result;
}