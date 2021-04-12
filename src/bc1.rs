//! BC1 (DXT1) Encoder.

use crate::color::{min_max_luminance, ColorRgba8};
use crate::utils::{encode_color_table_bc1_bc3, encode_image, encode_image_conv_u8, fetch_block};

struct MinMax<T: ColorRgba8> {
    min: T,
    min_565: u16,
    max: T,
    max_565: u16,
    can_contain_alpha: bool,
}

fn min_max_colors<T: ColorRgba8>(block: &[T]) -> MinMax<T> {
    let (min, max) = min_max_luminance(block);

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

pub fn encode_block_bc1<T: ColorRgba8>(block: [T; 16]) -> Block8 {
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

    Block8 {
        min: min_565,
        max: max_565,
        color_table: encode_color_table_bc1_bc3(&block, [max, min, color2, color3]),
    }
}

fn fetch_and_encode<T: ColorRgba8>(pixels: &[T], x: usize, y: usize, width: usize) -> Block8 {
    encode_block_bc1(fetch_block(pixels, x, y, width))
}

pub fn encode_image_bc1<T>(pixels: &[T], width: usize, height: usize) -> Vec<Block8>
where
    T: ColorRgba8,
{
    encode_image(pixels, width, height, fetch_and_encode)
}

pub fn encode_image_bc1_conv_u8<T>(pixels: &[T], width: usize, height: usize) -> Vec<u8>
where
    T: ColorRgba8,
{
    encode_image_conv_u8(pixels, width, height, fetch_and_encode)
}
