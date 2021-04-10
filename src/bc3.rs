//! BC3 (DXT5) Encoder.

use crate::{
    color::{min_max_luminance, ColorSource},
    encode_color_table_bc1_bc3, encode_image, encode_image_conv_u8, fetch_block,
};

struct MinMax<T: ColorSource> {
    min: T,
    min_565: u16,
    max: T,
    max_565: u16,
}

fn min_max_colors<T: ColorSource>(block: &[T]) -> MinMax<T> {
    let (min, max) = min_max_luminance(block);

    let min_565 = min.to_565();
    let max_565 = max.to_565();

    if max_565 > min_565 {
        MinMax {
            min,
            min_565,
            max,
            max_565,
        }
    } else {
        MinMax {
            min_565: max_565,
            max_565: min_565,
            min: max,
            max: min,
        }
    }
}

fn min_max_alpha<T: ColorSource>(block: &[T]) -> (u8, u8) {
    let mut min = u8::MAX;
    let mut max = 0;
    for p in block {
        let alpha = p.alpha();
        if alpha < min {
            min = alpha;
        }
        if alpha > max {
            max = alpha;
        }
    }
    (min, max)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Block16 {
    pub max_alpha: u8,
    pub min_alpha: u8,
    pub alpha_table: [u8; 6],
    pub max: u16,
    pub min: u16,
    pub color_table: u32,
}

#[inline(always)]
const fn mix_saturate(a_fraction: u32, a: u8, b_fraction: u32, b: u8) -> u8 {
    let v = (a_fraction * (a as u32) + b_fraction * (b as u32)) / (a_fraction + b_fraction);
    if v > 255 {
        255
    } else {
        v as u8
    }
}

fn gen_alpha_ref(min: u8, max: u8) -> [u8; 8] {
    [
        max,
        min,
        mix_saturate(6, min, 1, max),
        mix_saturate(5, min, 2, max),
        mix_saturate(4, min, 3, max),
        mix_saturate(3, min, 4, max),
        mix_saturate(2, min, 5, max),
        mix_saturate(1, min, 6, max),
    ]
}

pub fn encode_block_bc3<T: ColorSource>(block: [T; 16]) -> Block16 {
    let (min_alpha, max_alpha) = min_max_alpha(&block);

    let alpha_ref = gen_alpha_ref(min_alpha, max_alpha);

    let mut indices = [0u8; 16];
    for (p, alpha_index) in block.iter().zip(indices.iter_mut()) {
        let mut min_delta = i32::MAX;
        let alpha = p.alpha() as i32;
        for (ref_index, ref_alpha) in alpha_ref.iter().enumerate() {
            let delta = ((*ref_alpha as i32) - alpha).abs();
            if delta < min_delta {
                min_delta = delta;
                *alpha_index = ref_index as u8;
            }
        }
    }

    let alpha_table = [
        (indices[0] >> 0) | (indices[1] << 3) | (indices[2] << 6),
        (indices[2] >> 2) | (indices[3] << 1) | (indices[4] << 4) | (indices[5] << 7),
        (indices[5] >> 1) | (indices[6] << 2) | (indices[7] << 5),
        (indices[8] >> 0) | (indices[9] << 3) | (indices[10] << 6),
        (indices[10] >> 2) | (indices[11] << 1) | (indices[12] << 4) | (indices[13] << 7),
        (indices[13] >> 1) | (indices[14] << 2) | (indices[15] << 5),
    ];

    let MinMax {
        min,
        min_565,
        max,
        max_565,
    } = min_max_colors(&block);

    let color2 = max.mix_2_1_over_3_saturate(&min);
    let color3 = max.mix_1_2_over_3_saturate(&min);

    Block16 {
        max_alpha,
        min_alpha,
        alpha_table,
        max: max_565,
        min: min_565,
        color_table: encode_color_table_bc1_bc3(&block, [max, min, color2, color3]),
    }
}

fn fetch_and_encode<T: ColorSource>(pixels: &[T], x: usize, y: usize, width: usize) -> Block16 {
    encode_block_bc3(fetch_block(pixels, x, y, width))
}

pub fn encode_image_bc3<T>(pixels: &[T], width: usize, height: usize) -> Vec<Block16>
where
    T: ColorSource,
{
    encode_image(pixels, width, height, fetch_and_encode)
}

pub fn encode_image_bc3_conv_u8<T>(pixels: &[T], width: usize, height: usize) -> Vec<u8>
where
    T: ColorSource,
{
    encode_image_conv_u8(pixels, width, height, fetch_and_encode)
}
