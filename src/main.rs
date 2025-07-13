use clap::Parser;
use sha2::{Sha256, Digest};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about = "Distributed CPU Miner", long_about = None)]
struct Args {
    #[arg(long)]
    range_start: u64,

    #[arg(long)]
    range_end: u64,

    #[arg(long, default_value_t = 1)]
    step: u64,

    #[arg(long)]
    wallet: String,

    #[arg(long)]
    license_key: String,

    #[arg(long, default_value = "http://localhost:8000")] // for future orchestrator use
    orchestrator_url: String,
}

fn hash_nonce(nonce: u64, wallet: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(nonce.to_le_bytes());
    hasher.update(wallet.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn main() {
    let args = Args::parse();

    println!(
        "Starting miner for wallet {} with step {} from {} to {}",
        args.wallet, args.step, args.range_start, args.range_end
    );

    let start_time = Instant::now();
    let mut count = 0;

    for nonce in (args.range_start..args.range_end).step_by(args.step as usize) {
        let hash = hash_nonce(nonce, &args.wallet);
        count += 1;

        if hash.ends_with("0000") {
            println!("[FOUND] Nonce: {} Hash: {}", nonce, hash);
        }

        if count % 10_000 == 0 {
            println!("Checked {} nonces... latest: {}", count, nonce);
        }
    }

    let duration = Instant::now() - start_time;
    println!("Done. Checked {} nonces in {:.2?}.", count, duration);
}

