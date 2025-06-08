fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut time_formats = Vec::new();
    for hour in 0..12 {
        for minute in 0..60 {
            if (hour as i32).count_ones() + (minute as i32).count_ones() == turned_on as u32 {
                time_formats.push(format!("{}:{:02}", hour, minute));
            }
        }
    }
    time_formats
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(
            read_binary_watch(1),
            vec![
                String::from("0:01"),
                String::from("0:02"),
                String::from("0:04"),
                String::from("0:08"),
                String::from("0:16"),
                String::from("0:32"),
                String::from("1:00"),
                String::from("2:00"),
                String::from("4:00"),
                String::from("8:00"),
            ]
        );
        assert_eq!(read_binary_watch(9), Vec::<String>::new());
    }
}
