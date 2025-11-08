fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let (mut odd_sum, mut even_sum) = (vec![0; n], vec![0; n]);

    let mut total_sum = arr[0];
    odd_sum[0] = arr[0];

    for i in 1..n {
        odd_sum[i] = even_sum[i - 1] + arr[i] * ((i as i32 / 2) + 1);
        even_sum[i] = odd_sum[i - 1] + arr[i] * (((i + 1) as i32) / 2);
        total_sum += odd_sum[i];
    }
    total_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_odd_length_subarrays() {
        assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
