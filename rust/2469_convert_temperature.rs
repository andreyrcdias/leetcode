fn convert_temperature(celsius: f64) -> Vec<f64> {
    let kelvin = celsius + 273.15;
    let fahrenheit = celsius * 1.80 + 32.00;
    vec![kelvin, fahrenheit]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_temperature() {
        assert_eq!(convert_temperature(36.50), vec![309.65000, 97.70000]);
        assert_eq!(convert_temperature(122.11), vec![395.26000, 251.79800]);
    }
}
