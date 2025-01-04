use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct CollatzCache {
    cache: Mutex<HashMap<BigInt, Vec<BigInt>>>,
}

impl CollatzCache {
    fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn get(&self, n: &BigInt) -> Option<Vec<BigInt>> {
        let cache: std::sync::MutexGuard<'_, HashMap<BigInt, Vec<BigInt>>> =
            self.cache.lock().unwrap();
        cache.get(n).cloned()
    }

    fn insert(&self, n: BigInt, sequence: Vec<BigInt>) {
        let mut cache: std::sync::MutexGuard<'_, HashMap<BigInt, Vec<BigInt>>> =
            self.cache.lock().unwrap();
        cache.insert(n, sequence);
    }
}

/// calculates the Collatz sequence for a given number using caching
fn collatz_sequence(n: &BigInt, cache: &Arc<CollatzCache>) -> Vec<BigInt> {
    let mut sequence: Vec<BigInt> = Vec::new();
    let mut current: BigInt = n.clone();

    while current > BigInt::one() {
        sequence.push(current.clone());

        if let Some(cached_seq) = cache.get(&current) {
            sequence.extend(cached_seq);
            return sequence;
        }

        if &current % 2.to_bigint().unwrap() == BigInt::zero() {
            // when current is even
            current /= 2; // divide by 2
        } else {
            // when current is odd
            current = current * 3 + 1; // multiply by 3 and add 1
        }
    }

    sequence.push(BigInt::one());

    cache.insert(n.clone(), sequence[1..].to_vec());
    sequence
}

fn main() {
    use std::io;

    let cache: Arc<CollatzCache> = Arc::new(CollatzCache::new());
    let mut input: String = String::new();

    println!("Enter a number to calculate Collatz sequences up to:");
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();

    let num: BigInt = match input.parse::<BigInt>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number. Please enter a valid integer.");
            return;
        }
    };

    let mut i: BigInt = 1.to_bigint().unwrap();
    while i <= num {
        let sequence: Vec<BigInt> = collatz_sequence(&i, &cache);
        print!("Collatz sequence for {}: ", i);
        for step in sequence {
            print!("{} ", step);
        }
        println!();

        i += 1;
    }
}
