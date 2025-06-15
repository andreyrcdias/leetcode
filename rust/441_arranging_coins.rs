fn arrange_coins(n: i32) -> i32 {
    // quadratic equation: (-v +- sqrt(b^2 - 4ac))/2a
    // a = 1; b = 1; c = -2n
    // (-1 + sqrt(1+8n))/2
    ((-1.0 + (8.0 * n as f64 + 1.0).sqrt()) / 2.0).floor() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arrange_coins() {
        assert_eq!(arrange_coins(5), 2);
        assert_eq!(arrange_coins(8), 3);
    }
}
