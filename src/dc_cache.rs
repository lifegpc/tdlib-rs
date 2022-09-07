use crate::ext::mutex::GetMutex;
use bytes::BytesMut;
use futures_util::lock::Mutex;
use std::collections::HashMap;

/// The cache status of a number
pub enum PrimeCacheStatus {
    /// The number is a prime
    Good,
    /// The number is not a prime
    Bad,
    /// The number is not in the cache
    Miss,
}

/// Used to maintain some caches.
pub struct DcCache {
    /// The cache of prime numbers
    prime_caches: HashMap<BytesMut, bool>,
}

impl DcCache {
    /// Add a new number to the cache.
    pub fn add_prime(&mut self, prime: &BytesMut, is_good: bool) {
        self.prime_caches.insert(prime.clone(), is_good);
    }

    /// Get the cache status of a number.
    pub fn is_good_prime(&self, prime: &BytesMut) -> PrimeCacheStatus {
        match self.prime_caches.get(prime) {
            Some(v) => match v {
                true => PrimeCacheStatus::Good,
                false => PrimeCacheStatus::Bad,
            },
            None => PrimeCacheStatus::Miss,
        }
    }

    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            prime_caches: HashMap::new(),
        }
    }
}

lazy_static::lazy_static! {
    /// The global cache.
    pub static ref DC_CACHE: Mutex<DcCache> = Mutex::new(DcCache::new());
}

/// Get the cache status of a number.
pub fn is_good_prime(prime: &BytesMut) -> PrimeCacheStatus {
    DC_CACHE.get_mutex().is_good_prime(prime)
}

/// Add a new number to the cache.
pub fn add_prime(prime: &BytesMut, is_good: bool) {
    DC_CACHE.get_mutex().add_prime(prime, is_good)
}
