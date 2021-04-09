//! BC1 (DXT1) Encoder.

use crate::color::ColorSource;

struct MinMax<T: ColorSource> {
    min: T,
    min_565: u16,
    max: T,
    max_565: u16,
    can_contain_alpha: bool,
}

fn min_max_colors<T: ColorSource>(block: &[T]) -> MinMax<T> {
    let mut max_lum = -1;
    let mut min_lum = i32::MAX;
    let mut max = block[0];
    let mut min = block[0];

    for p in block {
        let lum = p.luminance();
        if lum > max_lum {
            max_lum = lum;
            max = *p;
        }
        if lum < min_lum {
            min_lum = lum;
            min = *p;
        }
    }

    let min_565 = min.to_565();
    let max_565 = max.to_565();

    if max_565 > min_565 {
        MinMax {
            min,
            min_565,
            max,
            max_565,
            can_contain_alpha: false,
        }
    } else {
        MinMax {
            min_565: max_565,
            max_565: min_565,
            min: max,
            max: min,
            can_contain_alpha: true,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Block8 {
    pub max: u16,
    pub min: u16,
    pub color_table: u32,
}

pub fn encode_block_bc1<T: ColorSource>(block: [T; 16]) -> Block8 {
    let MinMax {
        min,
        min_565,
        max,
        max_565,
        can_contain_alpha,
    } = min_max_colors(&block);

    let (color2, color3) = if T::contains_alpha() && can_contain_alpha {
        (max.mix_1_1_over_2_saturate(&min), T::default())
    } else {
        (
            max.mix_2_1_over_3_saturate(&min),
            max.mix_1_2_over_3_saturate(&min),
        )
    };

    let ref_colors = [max, min, color2, color3];

    // Find color indices and pack result.
    let mut color_indices = [0; 16];
    for (p, color_index) in block.iter().zip(color_indices.iter_mut()) {
        if T::contains_alpha() && p.alpha() < 128 {
            // Map to black.
            *color_index = 3;
        } else {
            let mut min_distance = i32::MAX;
            for (i, ref_color) in ref_colors.iter().enumerate() {
                let distance = p.sqr_distance(ref_color);
                if distance < min_distance {
                    *color_index = i;
                    min_distance = distance;
                }
            }
        }
    }

    let mut color_table = 0;
    for (i, index) in color_indices.iter().enumerate() {
        color_table |= (*index as u32) << (i << 1);
    }

    Block8 {
        min: min_565,
        max: max_565,
        color_table,
    }
}

fn fetch_or_default<T: ColorSource>(pixels: &[T], n: usize) -> T {
    match pixels.get(n) {
        Some(pixel) => *pixel,
        None => T::default(),
    }
}

fn fetch_and_encode<T: ColorSource>(pixels: &[T], x: usize, y: usize, width: usize) -> Block8 {
    let c0 = x;
    let c1 = x + 1;
    let c2 = x + 2;
    let c3 = x + 3;
    let r0 = y * width;
    let r1 = (y + 1) * width;
    let r2 = (y + 2) * width;
    let r3 = (y + 3) * width;

    let block = [
        // Row 0
        fetch_or_default(&pixels, c0 + r0),
        fetch_or_default(&pixels, c1 + r0),
        fetch_or_default(&pixels, c2 + r0),
        fetch_or_default(&pixels, c3 + r0),
        // Row 1
        fetch_or_default(&pixels, c0 + r1),
        fetch_or_default(&pixels, c1 + r1),
        fetch_or_default(&pixels, c2 + r1),
        fetch_or_default(&pixels, c3 + r1),
        // Row 2
        fetch_or_default(&pixels, c0 + r2),
        fetch_or_default(&pixels, c1 + r2),
        fetch_or_default(&pixels, c2 + r2),
        fetch_or_default(&pixels, c3 + r2),
        // Row 3
        fetch_or_default(&pixels, c0 + r3),
        fetch_or_default(&pixels, c1 + r3),
        fetch_or_default(&pixels, c2 + r3),
        fetch_or_default(&pixels, c3 + r3),
    ];

    encode_block_bc1(block)
}

fn ceil_div_4(x: usize) -> usize {
    (x + 3) / 4
}

pub fn encode_image<T: ColorSource>(pixels: &[T], width: usize, height: usize) -> Vec<Block8> {
    let w = ceil_div_4(width);
    let h = ceil_div_4(height);

    let mut encoded_pixels = Vec::with_capacity(w * h);

    for i in 0..h {
        let y = i * 4;
        for j in 0..w {
            let x = j * 4;
            encoded_pixels.push(fetch_and_encode(pixels, x, y, width));
        }
    }

    encoded_pixels
}

pub fn encode_image_conv_u8<T: ColorSource>(pixels: &[T], width: usize, height: usize) -> Vec<u8> {
    let mut compressed_pixels = encode_image(pixels, width, height);

    let transmuted = unsafe {
        Vec::from_raw_parts(
            compressed_pixels.as_mut_ptr() as *mut u8,
            compressed_pixels.len() * std::mem::size_of::<Block8>(),
            compressed_pixels.capacity() * std::mem::size_of::<Block8>(),
        )
    };

    // Explicitly forget because we're transmuting memory block.
    std::mem::forget(compressed_pixels);

    transmuted
}
