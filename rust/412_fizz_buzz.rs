fn fizz_buzz(n: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            ans.push(String::from("FizzBuzz"));
        } else if i % 3 == 0 {
            ans.push(String::from("Fizz"));
        } else if i % 5 == 0 {
            ans.push(String::from("Buzz"));
        } else {
            ans.push(format!("{}", i));
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            fizz_buzz(3),
            vec![String::from("1"), String::from("2"), String::from("Fizz")]
        );
        assert_eq!(
            fizz_buzz(5),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz")
            ]
        );
        assert_eq!(
            fizz_buzz(15),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz"),
                String::from("Fizz"),
                String::from("7"),
                String::from("8"),
                String::from("Fizz"),
                String::from("Buzz"),
                String::from("11"),
                String::from("Fizz"),
                String::from("13"),
                String::from("14"),
                String::from("FizzBuzz")
            ]
        );
    }
}
