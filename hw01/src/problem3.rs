use std::collections::HashSet;

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut crosser: HashSet<u32> = HashSet::new();
    for i in 2..n {
        if !crosser.contains(&i){
            primes.push(i);
            for j in i..n{
                crosser.insert(i * j);
            }
        }
        
    }
    return primes;
}