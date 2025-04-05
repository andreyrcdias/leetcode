fn climb_stairs(n: i32) -> i32 {
    let (mut p, mut q) = (1, 1);
    for _i in 1..n {
        let t = p + q;
        p = q;
        q = t;
    }
    q
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}
