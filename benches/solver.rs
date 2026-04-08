use std::time::{Duration, Instant};
use vercel_security_checkpoint_foss::solver::{solve_challenge, solve_challenge_faster};

const TOKEN: &str = "2.1775605412.60.OTI5Y2IwZDdmZWFlMDQwNDA0ZjNjYTUwMGVhOGRiZWM7ZjUxNDdjZDk7ZDAyMTFmNDUxMGNjOGEwY2YwODZlMDZhOTEyODZhYmUyZDkzZjg2NjszO8snHZn5V+mQOwOAbUW9CLMmEbk1gIEBvaNeAR53pS2W7sy6jqhmkn2y1w==.ba780fc4d5b43e3b4c6d7c41b28bca75";
const RNG_SEED: u64 = 58;
const WARMUP_ITERS: u32 = 5;
const BENCH_ITERS: u32 = 50;

fn bench(name: &str, f: impl Fn()) {
    for _ in 0..WARMUP_ITERS {
        f();
    }

    let mut times = Vec::with_capacity(BENCH_ITERS as usize);
    for _ in 0..BENCH_ITERS {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
    }

    times.sort();
    let sum: Duration = times.iter().sum();
    let avg = sum / BENCH_ITERS;
    let min = times[0];
    let max = times[times.len() - 1];
    let median = times[times.len() / 2];

    eprintln!(
        "{name:>30}  avg {:.2}ms  median {:.2}ms  min {:.2}ms  max {:.2}ms  ({BENCH_ITERS} iters)",
        avg.as_secs_f64() * 1000.0,
        median.as_secs_f64() * 1000.0,
        min.as_secs_f64() * 1000.0,
        max.as_secs_f64() * 1000.0,
    );
}

fn main() {
    bench("solve_challenge", || {
        std::hint::black_box(solve_challenge(std::hint::black_box(TOKEN), RNG_SEED));
    });
    bench("solve_challenge_faster", || {
        std::hint::black_box(solve_challenge_faster(std::hint::black_box(TOKEN)));
    });
}
