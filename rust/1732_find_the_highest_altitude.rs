use std::cmp::max;

fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut current_altitude = 0;
    let mut max_altitude = 0;
    for change in gain {
        current_altitude += change;
        max_altitude = max(max_altitude, current_altitude);
    }
    max_altitude
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_altitude() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
