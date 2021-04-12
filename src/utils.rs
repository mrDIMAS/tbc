use crate::color::ColorRgba8;

pub(in crate) fn ceil_div_4(x: usize) -> usize {
    (x + 3) / 4
}

pub(in crate) fn encode_color_table_bc1_bc3<T: ColorRgba8>(block: &[T], ref_colors: [T; 4]) -> u32 {
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

    color_table
}

pub(in crate) fn fetch_or_default<T>(pixels: &[T], n: usize) -> T
where
    T: Default + Copy,
{
    match pixels.get(n) {
        Some(pixel) => *pixel,
        None => T::default(),
    }
}

pub(in crate) fn fetch_block<T>(pixels: &[T], x: usize, y: usize, width: usize) -> [T; 16]
where
    T: Default + Copy,
{
    let c0 = x;
    let c1 = x + 1;
    let c2 = x + 2;
    let c3 = x + 3;
    let r0 = y * width;
    let r1 = (y + 1) * width;
    let r2 = (y + 2) * width;
    let r3 = (y + 3) * width;

    [
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
    ]
}

pub(in crate) fn encode_image<T, B, F>(
    pixels: &[T],
    width: usize,
    height: usize,
    fetch: F,
) -> Vec<B>
where
    F: Fn(&[T], usize, usize, usize) -> B,
{
    let w = ceil_div_4(width);
    let h = ceil_div_4(height);

    let mut encoded_pixels = Vec::with_capacity(w * h);

    for i in 0..h {
        let y = i * 4;
        for j in 0..w {
            let x = j * 4;
            encoded_pixels.push(fetch(pixels, x, y, width));
        }
    }

    encoded_pixels
}

pub(in crate) fn encode_image_conv_u8<T, B, F>(
    pixels: &[T],
    width: usize,
    height: usize,
    fetch: F,
) -> Vec<u8>
where
    F: Fn(&[T], usize, usize, usize) -> B,
{
    let mut compressed_pixels = encode_image(pixels, width, height, fetch);

    let transmuted = unsafe {
        Vec::from_raw_parts(
            compressed_pixels.as_mut_ptr() as *mut u8,
            compressed_pixels.len() * std::mem::size_of::<B>(),
            compressed_pixels.capacity() * std::mem::size_of::<B>(),
        )
    };

    // Explicitly forget because we're transmuting memory block.
    std::mem::forget(compressed_pixels);

    transmuted
}
