use std::time::SystemTime;

pub fn random_seed() -> u32 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|x| x.as_nanos() as u32).unwrap_or(1)
}

/// fast u32 hash function
pub fn khash(mut state: u32) -> u32 {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state
}

/// fast u32 hash function -> float [0..1]
pub fn fhash(mut state: u32) -> f32 {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state as f32 / 4294967295.0
}

/// sequence u32 hash function
pub fn next_u32(state: &mut u32) -> u32 {
    *state = (*state ^ 2747636419).wrapping_mul(2654435769);
    *state = (*state ^ (*state >> 16)).wrapping_mul(2654435769);
    *state = (*state ^ (*state >> 16)).wrapping_mul(2654435769);
    *state
}

/// sequence u32 hash function -> float [0..1]
pub fn next_f32(state: &mut u32) -> f32 {
    *state = (*state ^ 2747636419).wrapping_mul(2654435769);
    *state = (*state ^ (*state >> 16)).wrapping_mul(2654435769);
    *state = (*state ^ (*state >> 16)).wrapping_mul(2654435769);
    *state as f32 / 4294967295.0
}