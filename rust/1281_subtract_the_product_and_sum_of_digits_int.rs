fn subtract_product_and_sum(n: i32) -> i32 {
    let ns = n.to_string();
    println!("{}", ns);
    let nsv: Vec<i32> = ns
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    let product: i32 = nsv.iter().fold(1, |acc, x| acc * x);
    let sum: i32 = nsv.iter().sum();
    product - sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subtract_product_and_sum() {
        assert_eq!(subtract_product_and_sum(234), 15);
        assert_eq!(subtract_product_and_sum(4421), 21);
    }
}
