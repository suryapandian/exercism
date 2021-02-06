pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    let (mut count, mut num) = (0, 3);
    while count < n {
        let mut num_is_prime = true;
        for &prime in &primes {
            if num % prime == 0 {
                num_is_prime = false;
                break;
            }
        }
        if num_is_prime {
            primes.push(num);
            count += 1;
        }
        num += 2;
    }

    primes[n as usize] as u32
}
