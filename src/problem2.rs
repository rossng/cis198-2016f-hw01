pub fn sieve(n: u32) -> Vec<u32> {
    let mut candidates = vec![true; (n+1) as usize];
    let mut primes: Vec<u32> = vec![];
    for i in 2..n+1 {
        if candidates[i as usize] {
            primes.push(i);
        }
        let mut j = i*i;
        while j <= n {
            candidates[j as usize] = false;
            j += i;
        }
    }
    primes
}
