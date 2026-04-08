use base64::Engine;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};

const MULTIPLIERS: [i128; 5] = [498787, 533737, 619763, 708403, 828071];

pub fn solve_challenge(token: &str, rng_seed: u64) -> String {
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

    let mut current_target = &target[key..];
    let mut results = Vec::new();
    let mut rng = SmallRng::seed_from_u64(rng_seed);

    for i in 0..num_iter {
        loop {
            let h = random_hex_seeded(16, &mut rng);
            let candidate = format!("{}{}", prefix, h);
            let hash = sha256hex(&candidate);

            if hash[..4] == current_target[..4] {
                let next_offset = (((combined * MULTIPLIERS[i % 5]) % 60 + 60) % 60) as usize;
                let owned_hash = hash;
                results.push(h);
                current_target = Box::leak(owned_hash[next_offset..].to_string().into_boxed_str());
                break;
            }
        }
    }

    results.join(";")
}

fn sha256hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn random_hex_seeded(len: usize, rng: &mut SmallRng) -> String {
    let bytes_needed = (len + 1) / 2;
    let bytes: Vec<u8> = (0..bytes_needed).map(|_| rng.random()).collect();
    hex::encode(bytes)[..len].to_string()
}

/// This algorithm produces valid, but different results than browser.
/// For coolness I completely removed rng and replaced it with a counter, so solutions look like:
/// - dc3f000000000000;43a0000000000000;0f93010000000000
/// Instead of:
/// - 7c656c47e6fd4d47;c301042947873e92;e035792cf1266033
///
/// Can be easily fixed by initializing counter to a random value and
/// copying it to rng buffer instead of ``faster_hex::hex_encode(&counter.to_le_bytes(), ...``
pub fn solve_challenge_faster(token: &str) -> String {
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

    let mut current_target = [0u8; 2];
    faster_hex::hex_decode(target[key..key + 4].as_bytes(), &mut current_target).unwrap();

    let mut results = Vec::new();
    let mut counter: u64 = 0;

    let mut base_hasher: Sha256 = Sha256::new();
    base_hasher.update(prefix.as_bytes());

    let mut hex_buffer = [0u8; 16];
    let mut hash_buffer = [0u8; 64];

    for i in 0..num_iter {
        loop {
            let h = faster_hex::hex_encode(&counter.to_le_bytes(), &mut hex_buffer).unwrap();

            counter += 1;
            let mut hasher = base_hasher.clone();
            hasher.update(h.as_bytes());
            let hash = hasher.finalize();

            if hash[..2] == current_target {
                let next_offset = (((combined * MULTIPLIERS[i % 5]) % 60 + 60) % 60) as usize;
                let owned_hash = faster_hex::hex_encode(&hash, &mut hash_buffer).unwrap();
                results.push(h.to_string());
                faster_hex::hex_decode(
                    owned_hash[next_offset..next_offset + 4].as_bytes(),
                    &mut current_target,
                )
                .unwrap();
                break;
            }
        }
    }

    results.join(";")
}
