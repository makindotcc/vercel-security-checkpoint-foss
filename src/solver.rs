use base64::Engine;
use rand::Rng;
use sha2::{Digest, Sha256};

const MULTIPLIERS: [i128; 5] = [498787, 533737, 619763, 708403, 828071];

pub fn solve_challenge(token: &str) -> String {
    let dot_parts: Vec<&str> = token.split('.').collect();
    let b64 = dot_parts[3];
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .unwrap();
    let fields: Vec<&[u8]> = decoded.splitn(5, |&b| b == b';').collect();
    let field = |i: usize| std::str::from_utf8(fields[i]).unwrap();

    let seed: i128 = dot_parts[1].parse().unwrap();
    let combined = seed;
    let table_index = (((combined % 5) + 5) % 5) as usize;
    let key = (((combined * MULTIPLIERS[table_index]) % 36 + 36) % 36) as usize;

    let num_iter: usize = field(3).parse().unwrap();
    let prefix = field(1);
    let target = field(2);

    println!(
        "    fields: {}, iter={}, prefix=\"{}\", key={}",
        fields.len(),
        num_iter,
        prefix,
        key
    );

    let mut current_target = &target[key..];
    let mut results = Vec::new();

    for i in 0..num_iter {
        let mut attempts = 0u64;
        loop {
            let h = random_hex(16);
            let candidate = format!("{}{}", prefix, h);
            let hash = sha256hex(&candidate);
            attempts += 1;

            if hash[..4] == current_target[..4] {
                let next_offset = (((combined * MULTIPLIERS[i % 5]) % 60 + 60) % 60) as usize;
                let owned_hash = hash;
                println!("    iter {}: {} attempts, hex={}", i, attempts, h);
                results.push(h);
                current_target = Box::leak(owned_hash[next_offset..].to_string().into_boxed_str());
                break;
            }
        }
    }

    results.join(";")
}

fn random_hex(len: usize) -> String {
    let mut rng = rand::rng();
    let bytes_needed = (len + 1) / 2;
    let bytes: Vec<u8> = (0..bytes_needed).map(|_| rng.random()).collect();
    hex::encode(bytes)[..len].to_string()
}

fn sha256hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
