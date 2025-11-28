fn interpret(command: String) -> String {
    let result = command.replace("()", "o");
    result.replace("(al)", "al")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpret() {
        assert_eq!(interpret("G()(al)".to_string()), "Goal".to_string());
        assert_eq!(
            interpret("G()()()()(al)".to_string()),
            "Gooooal".to_string()
        );
        assert_eq!(
            interpret("(al)G(al)()()G".to_string()),
            "alGalooG".to_string()
        );
    }
}
