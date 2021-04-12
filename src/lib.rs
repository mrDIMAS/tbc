//! Texture Block Compression
//!
//! Pure Rust implementation of BCn texture compression algorithm implementations.
//!
//! Supported formats:
//! - BC1 (DXT1)
//! - BC3 (DTX5)
//! - BC4 (Both R8 and RG8)
//!
//! References:
//!
//! https://docs.microsoft.com/en-us/windows/win32/direct3d10/d3d10-graphics-programming-guide-resources-block-compression
//! https://www.researchgate.net/publication/259000525_Real-Time_DXT_Compression
//! https://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/
//! https://www.khronos.org/registry/DataFormat/specs/1.3/dataformat.1.3.html

pub mod bc1;
pub mod bc3;
pub mod bc4;
pub mod color;
pub mod utils;

pub use crate::{
    bc1::{encode_image_bc1, encode_image_bc1_conv_u8},
    bc3::{encode_image_bc3, encode_image_bc3_conv_u8},
    bc4::{
        encode_image_bc4_r8, encode_image_bc4_r8_conv_u8, encode_image_bc4_rg8,
        encode_image_bc4_rg8_conv_u8,
    },
};

#[cfg(test)]
mod tests {
    use crate::bc1::{encode_block_bc1, Block8};
    use crate::color::Rgba8;

    #[test]
    fn test_encode_block() {
        let block = [
            // First row.
            Rgba8::new(0, 0, 0, 0),
            Rgba8::new(64, 0, 0, 0),
            Rgba8::new(128, 0, 0, 0),
            Rgba8::new(255, 0, 0, 0),
            // Second row.
            Rgba8::new(0, 0, 0, 0),
            Rgba8::new(64, 0, 0, 0),
            Rgba8::new(128, 0, 0, 0),
            Rgba8::new(255, 0, 0, 0),
            // Third row.
            Rgba8::new(0, 0, 0, 0),
            Rgba8::new(64, 0, 0, 0),
            Rgba8::new(128, 0, 0, 0),
            Rgba8::new(255, 0, 0, 0),
            // Fourth row.
            Rgba8::new(0, 0, 0, 0),
            Rgba8::new(64, 0, 0, 0),
            Rgba8::new(128, 0, 0, 0),
            Rgba8::new(255, 0, 0, 0),
        ];

        assert_eq!(
            encode_block_bc1(block),
            Block8 {
                max: 63488,
                min: 0,
                color_table: 4294967295,
            }
        );
    }
}
