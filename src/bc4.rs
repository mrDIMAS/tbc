use crate::color::{ColorRed8, ColorRedGreen8};
use crate::utils::{encode_image, encode_image_conv_u8, fetch_block};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct GreyScaleBlock8 {
    pub max_alpha: u8,
    pub min_alpha: u8,
    pub alpha_table: [u8; 6],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct GreyScaleBlock16 {
    pub r: GreyScaleBlock8,
    pub g: GreyScaleBlock8,
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

fn min_max_alpha<T, F>(block: &[T], fetch: &F) -> (u8, u8)
where
    F: Fn(&T) -> u8,
{
    let mut min = u8::MAX;
    let mut max = 0;
    for p in block {
        let alpha = fetch(p);
        if alpha < min {
            min = alpha;
        }
        if alpha > max {
            max = alpha;
        }
    }
    (min, max)
}

pub(in crate) fn encode_block_bc4<T, F>(block: &[T], fetch: &F) -> GreyScaleBlock8
where
    F: Fn(&T) -> u8,
{
    let (min_alpha, max_alpha) = min_max_alpha(&block, fetch);

    let alpha_ref = gen_alpha_ref(min_alpha, max_alpha);

    let mut indices = [0u8; 16];
    for (p, alpha_index) in block.iter().zip(indices.iter_mut()) {
        let mut min_delta = i32::MAX;
        let alpha = fetch(p) as i32;
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

    GreyScaleBlock8 {
        max_alpha,
        min_alpha,
        alpha_table,
    }
}

fn fetch_and_encode_r8<T: ColorRed8>(
    pixels: &[T],
    x: usize,
    y: usize,
    width: usize,
) -> GreyScaleBlock8 {
    encode_block_bc4(&fetch_block(pixels, x, y, width), &|v| v.red())
}

pub fn encode_image_bc4_r8<T: ColorRed8>(
    pixels: &[T],
    width: usize,
    height: usize,
) -> Vec<GreyScaleBlock8> {
    encode_image(pixels, width, height, fetch_and_encode_r8)
}

pub fn encode_image_bc4_r8_conv_u8<T: ColorRed8>(
    pixels: &[T],
    width: usize,
    height: usize,
) -> Vec<u8> {
    encode_image_conv_u8(pixels, width, height, fetch_and_encode_r8)
}

fn fetch_and_encode_rg8<T: ColorRedGreen8>(
    pixels: &[T],
    x: usize,
    y: usize,
    width: usize,
) -> GreyScaleBlock16 {
    let block = fetch_block(pixels, x, y, width);
    let r = encode_block_bc4(&block, &|v| v.red());
    let g = encode_block_bc4(&block, &|v| v.green());
    GreyScaleBlock16 { r, g }
}

pub fn encode_image_bc4_rg8<T: ColorRedGreen8>(
    pixels: &[T],
    width: usize,
    height: usize,
) -> Vec<GreyScaleBlock16> {
    encode_image(pixels, width, height, fetch_and_encode_rg8)
}

pub fn encode_image_bc4_rg8_conv_u8<T: ColorRedGreen8>(
    pixels: &[T],
    width: usize,
    height: usize,
) -> Vec<u8> {
    encode_image_conv_u8(pixels, width, height, fetch_and_encode_rg8)
}
