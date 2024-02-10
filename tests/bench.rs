use std::time::Instant;
use std::path::Path;

use minimg::ImageBuffer;
use minirng::hash::*;
use minvect::*;

pub const SAMPLE_S: usize = 2000;
pub const BENCH_RUNS: usize = 1000000000;
#[test]
pub fn bench_khash() {
    let tstart = Instant::now();
    let seed = &mut random_seed();
    let mut acc = 0.0;
    for _ in 0..BENCH_RUNS {
        let f = next_f32(seed);
        acc += f;
    }
    let dur = tstart.elapsed().as_secs_f32();
    println!("bench khash took {}, {}", dur, acc);
}

#[test]
pub fn sample_khash() {
    let tstart = Instant::now();
    let seed = &mut random_seed();
    let mut buf = ImageBuffer::new(SAMPLE_S, SAMPLE_S);
    for i in 0..SAMPLE_S {
        for j in 0..SAMPLE_S {
            buf.set(i, j, vec4(
                next_f32(seed),
                next_f32(seed),
                next_f32(seed),
                1.0
            ));
        }
    }
    let dur = tstart.elapsed().as_secs_f32();
    println!("test khash took {}", dur);
    buf.save(Path::new("khash.png"));
}