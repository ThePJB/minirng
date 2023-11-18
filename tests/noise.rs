use std::time::Instant;

use minimg::ImageBuffer;
use minirng::{hash::*, noise::noise_grad};
use minvect::*;


const NOISE_S: usize = 2000;

#[test]
fn test_noise() {
    let tstart = Instant::now();
    let seed = &mut random_seed();
    let seed1 = next_u32(seed);
    let seed2 = next_u32(seed);
    let seed3 = next_u32(seed);
    let mut buf = ImageBuffer::new(NOISE_S, NOISE_S);
    for i in 0..NOISE_S {
        for j in 0..NOISE_S {
            let p = vec2(i as f32, j as f32) / NOISE_S as f32;
            let p = p - vec2(0.5, 0.5);
            let p = p * 16.0;
            buf.set(i, j, vec4(
                noise_grad(&p, seed1),
                noise_grad(&p, seed2),
                noise_grad(&p, seed3),
                1.0
            ));
        }
    }
    let dur = tstart.elapsed().as_secs_f32();
    println!("test noise took {}", dur);
    buf.dump_to_file("noise.png");
}