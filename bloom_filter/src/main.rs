use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct BloomFilter {
    bits: Vec<u64>, 
    size: usize, // total number of logical bits
    hash_count: usize, // number of hash functions to simulate
}

impl BloomFilter {
    fn new(size: usize, hash_count: usize) -> Self {
        let words = (size + 63) / 64; // ceiling division
        Self {
            bits: vec![0u64; words], // vector of words all initialized to zero
            size,
            hash_count,
        }
    }
    
    // saving bits as (word index, bit offset)
    fn set_bit(&mut self, pos: usize) {
        let word = pos / 64;
        let bit = pos % 64;
        self.bits[word] |= 1u64 << bit;
    }

    // move target bit to right and mask everything else, check if remaining value is 1
    fn get_bit(&self, pos: usize) -> bool {
        let word = pos / 64;
        let bit = pos % 64;
        (self.bits[word] >> bit) & 1 == 1
    }


    fn hash<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish() ^ seed.wrapping_mul( 0x9e3779b97f4a7c15); // number related to golden ration spreads hash nicely
        hash as usize % self.size
    }

    fn insert<T: Hash>(&mut self, item: &T) {
        for seed in 0..self.hash_count as u64 {
            let index = self.hash(item, seed);

            self.set_bit(index); // use the helper
        }
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        for seed in 0..self.hash_count as u64 {
            let index = self.hash(item, seed);

            if !self.get_bit(index) {
                return false; // if required bit is false -> definitely not present
            }
        }

        true // else maybe present
    }
}

fn main() {
    let mut filter = BloomFilter::new(10, 3);

    for (i, word) in filter.bits.iter().enumerate() {
        println!("word {}: {:064b}", i, word);
    }

    filter.insert(&"apple");

    println!("apple? {}", filter.contains(&"apple"));
    println!("banana? {}", filter.contains(&"banana"));

    println!("{:?}", filter.bits);
}
