#![cfg_attr(feature = "simd", feature(portable_simd))]

pub const N: usize = 1 << 22; // ~4M elements

#[cfg(not(feature = "simd"))]
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    for ((a, b), out) in a.iter().zip(b).zip(out) {
        *out = *a + *b;
    }
}

#[cfg(feature = "simd")]
pub fn add(a: &[f32], b: &[f32], out: &mut [f32]) {
    use std::simd::f32x8;
    const LANES: usize = 8;
    a.chunks_exact(LANES)
        .zip(b.chunks_exact(LANES))
        .zip(out.chunks_exact_mut(LANES))
        .for_each(|((a, b), out)| {
            let a = f32x8::from_slice(a);
            let b = f32x8::from_slice(b);
            (a + b).copy_to_slice(out);
        });
}
