//! Texture Block Compression
//!
//! Pure Rust implementation of BC1, BC5 texture compression algorithm
//!
//! References:
//!
//! https://docs.microsoft.com/en-us/windows/win32/direct3d10/d3d10-graphics-programming-guide-resources-block-compression
//! https://www.researchgate.net/publication/259000525_Real-Time_DXT_Compression
//! https://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/

pub mod bc1;
pub mod color;

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
