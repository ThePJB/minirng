use std::f32::consts::PI;
use minvect::*;
use crate::hash::*;

pub fn lerp(x: f32, y: f32, t: f32) -> f32 {
    x * (1.0 - t) + y * t
}

pub fn fade(t: f32) -> f32 {
    t*t*t*(t*(t*6.0-15.0)+10.0)
}

/// output range is 0..1 *may not be 0..1
pub fn noise_grad(p: &Vec2, seed: u32) -> f32 {
    // grid corners get a random angle unit vector
    // blend the vectors
    let p00 = vec2(p.x.floor(), p.y.floor());
    let p01 = vec2(p.x.floor(), p.y.ceil());
    let p11 = vec2(p.x.ceil(), p.y.ceil());
    let p10 = vec2(p.x.ceil(), p.y.floor());

    // half u32 thing may be unnecessary
    let half_u32 = (u32::MAX >> 1) as i32;

    let mut rng = seed;
    let ss = next_u32(&mut rng);
    let mut s00 = khash(seed.wrapping_add(((p00.x as i32).wrapping_add(half_u32)) as u32).wrapping_add(ss.wrapping_mul(((p00.y as i32).wrapping_add(half_u32)) as u32)));
    let mut s01 = khash(seed.wrapping_add(((p01.x as i32).wrapping_add(half_u32)) as u32).wrapping_add(ss.wrapping_mul(((p01.y as i32).wrapping_add(half_u32)) as u32)));
    let mut s10 = khash(seed.wrapping_add(((p10.x as i32).wrapping_add(half_u32)) as u32).wrapping_add(ss.wrapping_mul(((p10.y as i32).wrapping_add(half_u32)) as u32)));
    let mut s11 = khash(seed.wrapping_add(((p11.x as i32).wrapping_add(half_u32)) as u32).wrapping_add(ss.wrapping_mul(((p11.y as i32).wrapping_add(half_u32)) as u32)));

    // makes weird artifacts at the axes
    // let mut s00 = khash(seed.wrapping_add(p00.x.to_bits().wrapping_add(ss.wrapping_mul(p00.y.to_bits()))));
    // let mut s01 = khash(seed.wrapping_add(p01.x.to_bits().wrapping_add(ss.wrapping_mul(p01.y.to_bits()))));
    // let mut s10 = khash(seed.wrapping_add(p10.x.to_bits().wrapping_add(ss.wrapping_mul(p10.y.to_bits()))));
    // let mut s11 = khash(seed.wrapping_add(p11.x.to_bits().wrapping_add(ss.wrapping_mul(p11.y.to_bits()))));

    let a00 = 2.0*PI*next_f32(&mut s00);
    let a01 = 2.0*PI*next_f32(&mut s01);
    let a10 = 2.0*PI*next_f32(&mut s10);
    let a11 = 2.0*PI*next_f32(&mut s11);

    let u00 = vec2(a00.cos(), a00.sin());
    let u01 = vec2(a01.cos(), a01.sin());
    let u10 = vec2(a10.cos(), a10.sin());
    let u11 = vec2(a11.cos(), a11.sin());

    let m00 = next_f32(&mut s00);
    let m01 = next_f32(&mut s01);
    let m10 = next_f32(&mut s10);
    let m11 = next_f32(&mut s11);

    let s00 = (*p - p00).dot(m00*u00);
    let s01 = (*p - p01).dot(m01*u01);
    let s10 = (*p - p10).dot(m10*u10);
    let s11 = (*p - p11).dot(m11*u11);

    let fx = p.x.fract();
    let fy = p.y.fract();
    let fx = if fx < 0.0 {
        1.0 - fx.abs()
    } else {
        fx
    };
    let fy = if fy < 0.0 {
        1.0 - fy.abs()
    } else {
        fy
    };

    let tx = fade(fx);
    let ty = fade(fy);

    let ly0 = lerp(s00, s01, ty);
    let ly1 = lerp(s10, s11, ty);
    let l = lerp(ly0, ly1, tx);

    return (l + 0.5).max(0.0).min(1.0);
}

pub fn noise_exp(p: &Vec2, seed: u32) -> f32 {
    -noise_grad(p, seed).ln()
}