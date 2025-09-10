fn count_primes(n: i32) -> i32 {
    // counting prime numbers using Sieve of Eratosthenes
    let mut is_prime = vec![true; (n + 1) as usize];
    if n >= 0 {
        is_prime[0] = false;
    }
    if n >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=(n as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i)..=(n as usize) {
                if j % i == 0 {
                    is_prime[j] = false;
                }
            }
        }
    }

    is_prime.iter().filter(|&p| *p).count() as i32
}

fn num_prime_arrangements(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let prime_count = count_primes(n);
    let non_prime_count = n - prime_count;

    let factorial = |mut num: i32| -> i64 {
        let mut res = 1;
        while num > 1 {
            res = (res * num as i64) % MOD;
            num -= 1;
        }
        res
    };

    let result = (factorial(prime_count) * factorial(non_prime_count)) % MOD;
    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_prime_arrangements() {
        assert_eq!(num_prime_arrangements(5), 12);
        assert_eq!(num_prime_arrangements(100), 682289015);
    }
}
