use std::f32::consts::PI;
use minvect::*;
use crate::hash::*;

pub fn lerp(x: f32, y: f32, t: f32) -> f32 {
    x * (1.0 - t) + y * t
}

pub fn fade(t: f32) -> f32 {
    t*t*t*(t*(t*6.0-15.0)+10.0)
}

pub fn noise_grad(p: &Vec2, seed: u32) -> f32 {
    // grid corners get a random vector
    // blend the vectors
    let i = vec2(1.0, 0.0);
    let j = vec2(0.0, 1.0);

    let p00 = vec2(p.x.floor(), p.y.floor());
    let p01 = p00 + j;
    let p10 = p00 + i;
    let p11 = p00 + i + j;

    let a00 = 2.0*PI*fhash(seed.wrapping_add(1512347u32.wrapping_mul(p00.x as u32).wrapping_add(213154127u32.wrapping_mul(p00.y as u32))));
    let a01 = 2.0*PI*fhash(seed.wrapping_add(1512347u32.wrapping_mul(p01.x as u32).wrapping_add(213154127u32.wrapping_mul(p01.y as u32))));
    let a10 = 2.0*PI*fhash(seed.wrapping_add(1512347u32.wrapping_mul(p10.x as u32).wrapping_add(213154127u32.wrapping_mul(p10.y as u32))));
    let a11 = 2.0*PI*fhash(seed.wrapping_add(1512347u32.wrapping_mul(p11.x as u32).wrapping_add(213154127u32.wrapping_mul(p11.y as u32))));

    let u00 = vec2(a00.cos(), a00.sin());
    let u01 = vec2(a01.cos(), a01.sin());
    let u10 = vec2(a10.cos(), a10.sin());
    let u11 = vec2(a11.cos(), a11.sin());

    let m00 = fhash(seed.wrapping_add(467897247u32.wrapping_mul(p00.x as u32).wrapping_add(3195781247u32.wrapping_mul(p00.y as u32))));
    let m01 = fhash(seed.wrapping_add(467897247u32.wrapping_mul(p01.x as u32).wrapping_add(3195781247u32.wrapping_mul(p01.y as u32))));
    let m10 = fhash(seed.wrapping_add(467897247u32.wrapping_mul(p10.x as u32).wrapping_add(3195781247u32.wrapping_mul(p10.y as u32))));
    let m11 = fhash(seed.wrapping_add(467897247u32.wrapping_mul(p11.x as u32).wrapping_add(3195781247u32.wrapping_mul(p11.y as u32))));

    let s00 = (*p - p00).dot(m00*u00);
    let s01 = (*p - p01).dot(m01*u01);
    let s10 = (*p - p10).dot(m10*u10);
    let s11 = (*p - p11).dot(m11*u11);

    let fx = p.x.fract();
    let fy = p.y.fract();

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