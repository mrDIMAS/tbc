//! BC3 (DXT5) Encoder.

use crate::{
    bc4::{encode_block_bc4, GreyScaleBlock8},
    color::{min_max_luminance, ColorRgba8},
    utils::{encode_color_table_bc1_bc3, encode_image, encode_image_conv_u8, fetch_block},
};

struct MinMax<T: ColorRgba8> {
    min: T,
    min_565: u16,
    max: T,
    max_565: u16,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Block16 {
    pub alpha_block: GreyScaleBlock8,
    pub max: u16,
    pub min: u16,
    pub color_table: u32,
}

pub fn encode_block_bc3<T: ColorRgba8>(block: [T; 16]) -> Block16 {
    let alpha_block = encode_block_bc4(&block, &|p| p.alpha());

    let MinMax {
        min,
        min_565,
        max,
        max_565,
    } = min_max_colors(&block);

    let color2 = max.mix_2_1_over_3_saturate(&min);
    let color3 = max.mix_1_2_over_3_saturate(&min);

    Block16 {
        alpha_block,
        max: max_565,
        min: min_565,
        color_table: encode_color_table_bc1_bc3(&block, [max, min, color2, color3]),
    }
}

fn fetch_and_encode<T: ColorRgba8>(pixels: &[T], x: usize, y: usize, width: usize) -> Block16 {
    encode_block_bc3(fetch_block(pixels, x, y, width))
}

pub fn encode_image_bc3<T>(pixels: &[T], width: usize, height: usize) -> Vec<Block16>
where
    T: ColorRgba8,
{
    encode_image(pixels, width, height, fetch_and_encode)
}

pub fn encode_image_bc3_conv_u8<T>(pixels: &[T], width: usize, height: usize) -> Vec<u8>
where
    T: ColorRgba8,
{
    encode_image_conv_u8(pixels, width, height, fetch_and_encode)
}
