fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].clone();
    for str in &strs[1..] {
        while !str.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }
    prefix
}

fn main() {
    // let strs = vec!["dog", "racecar", "car"]
    let strs = vec!["flower", "flow", "flight"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", longest_common_prefix(strs));
}
