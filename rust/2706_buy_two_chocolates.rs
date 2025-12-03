fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;

    for &price in prices.iter() {
        if price < min1 {
            min2 = min1;
            min1 = price;
        } else if price < min2 {
            min2 = price;
        }
    }
    let total = min1 + min2;
    if money < total {
        money
    } else {
        money - total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_buy_choco() {
        assert_eq!(buy_choco(vec![1, 2, 3], 3), 0);
        assert_eq!(buy_choco(vec![3, 2, 3], 3), 3);
    }
}
