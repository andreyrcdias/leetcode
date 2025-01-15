fn is_palindrom(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            stack.push(ch.to_ascii_lowercase());
        }
    }

    for ch in s.chars() {
        if let Some(&top_char) = stack.last() {
            if top_char == ch.to_ascii_lowercase() {
                stack.pop();
            }
        }
    }

    stack.len() == 0
}

fn main() {
    let input_string = "A man, a plan, a canal: Panama";
    // let input_stirng = "race a car";
    // let input_stirng = " ";

    println!("Palindrom: {}", is_palindrom(input_string.to_string()));
}
