pub fn build_proverb(list: Vec<&str>) -> String {
    let mut result = Vec::with_capacity(list.len() + 1);
    if list.len() == 0 {
        return String::new();
    }
    for i in 0..list.len() - 1 {
        result.push(format!("For want of a {0} the {1} was lost.", list[i], list[i + 1]));
    }
    result.push(format!("And all for the want of a {0}.", list[0]));
    return result.join("\n");
}
