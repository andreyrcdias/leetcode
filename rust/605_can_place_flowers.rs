fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let m = flowerbed.len();
    let mut fb = flowerbed.to_vec();
    let mut n = n;

    for i in 0..m {
        let l = if i == 0 { 0 } else { fb[i - 1] };
        let r = if i == m - 1 { 0 } else { fb[i + 1] };

        if l + fb[i] + r == 0 {
            fb[i] = 1;
            n -= 1;
        }
    }

    n <= 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }
}
