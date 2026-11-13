use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::time::Instant;
use: std::time::Instant:

fn main() {
    println!("================================");
    println!("       M5 SECURITY LAB");
    println!("================================");

    let cores = num_cpus::get();

    println!("Logical CPU threads: {}", cores);
    println!("Starting benchmark...\n");

    let start = Instant::now();

    let total: u64 = (0..5_000_000_u64)
        .into_par_iter()
        .map(|i| {
            let input = format!("synthetic-test-{}", i);
            let hash = Sha256::digest(input.as_bytes());
            hash[0] as u64
        })
        .sum();

    println!("Benchmark completed!");
    println!("Checksum: {}", total);
    println!("Execution time: {:.2?}", start.elapsed());

    println!("\nSecurity demonstration:");

    let sample = "Example-Password-2026!";

    let hash = Sha256::digest(sample.as_bytes());

    println!("Sample password length: {}", sample.len());
    println!("SHA-256 hash: {:x}", hash);

    println!("\nM5 SECURITY LAB FINISHED");
}
