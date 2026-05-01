//! Pixel processing — auto-vectorized via LLVM, no explicit SIMD needed.

pub const ALPHA_THRESHOLD: u8 = 30;

#[inline]
#[must_use]
pub fn luma_scalar(r: u8, g: u8, b: u8) -> u32 {
    (2126 * u32::from(r) + 7152 * u32::from(g) + 722 * u32::from(b)) / 10000
}

/// Scan RGBA bytes and return `(min, max)` luma of opaque pixels.
/// Returns `(u32::MAX, u32::MIN)` if no opaque pixel found.
#[must_use]
pub fn find_luma_range_rgba_bytes(bytes: &[u8]) -> (u32, u32) {
    let mut min = u32::MAX;
    let mut max = u32::MIN;

    for pixel in bytes.chunks_exact(4) {
        // Safety: chunks_exact(4) guarantees length
        let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
        if a >= ALPHA_THRESHOLD {
            let luma = luma_scalar(r, g, b);
            min = min.min(luma);
            max = max.max(luma);
        }
    }
    (min, max)
}

/// Compute charset index for each pixel in a 32-byte (8 pixel) RGBA chunk.
/// Returns `(luma_index, is_opaque)` per pixel.
#[must_use]
pub fn compute_charset_indices(
    chunk: &[u8; 32],
    luma_min: u32,
    luma_range: u32,
    num_chars_minus_1: u32,
) -> [(u32, bool); 8] {
    let mut out = [(0u32, false); 8];
    for i in 0..8 {
        let base = i * 4;
        let (r, g, b, a) = (
            chunk[base],
            chunk[base + 1],
            chunk[base + 2],
            chunk[base + 3],
        );
        let luma = luma_scalar(r, g, b);
        let norm = (luma.saturating_sub(luma_min) * 255) / luma_range;
        let idx = (norm * num_chars_minus_1 / 255).min(num_chars_minus_1);
        out[i] = (idx, a >= ALPHA_THRESHOLD);
    }
    out
}
