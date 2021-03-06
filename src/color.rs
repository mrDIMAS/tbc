#[inline(always)]
fn mix_2_1_over_3_saturate(x: u8, y: u8) -> u8 {
    ((2 * (x as u16) + y as u16) / 3).min(255) as u8
}

#[inline(always)]
fn mix_1_2_over_3_saturate(x: u8, y: u8) -> u8 {
    ((x as u16 + 2 * (y as u16)) / 3).min(255) as u8
}

#[inline(always)]
fn mix_1_1_over_2_saturate(x: u8, y: u8) -> u8 {
    ((x as u16 + y as u16) / 2).min(255) as u8
}

pub trait ColorRgba8: Copy + Clone + Default {
    fn to_565(&self) -> u16;

    fn luminance(&self) -> i32;

    fn sqr_distance(&self, other: &Self) -> i32;

    fn mix_2_1_over_3_saturate(&self, other: &Self) -> Self;

    fn mix_1_2_over_3_saturate(&self, other: &Self) -> Self;

    fn mix_1_1_over_2_saturate(&self, other: &Self) -> Self;

    fn contains_alpha() -> bool;

    fn alpha(&self) -> u8;
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Rgba8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba8 {
    #[inline(always)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl ColorRgba8 for Rgba8 {
    #[inline(always)]
    fn to_565(&self) -> u16 {
        (((self.r & 0b11111000) as u16) << 8)
            + (((self.g & 0b11111100) as u16) << 3)
            + (self.b >> 3) as u16
    }

    #[inline(always)]
    fn luminance(&self) -> i32 {
        self.r as i32 + (self.g as i32) * 2 + self.b as i32
    }

    #[inline(always)]
    fn sqr_distance(&self, other: &Self) -> i32 {
        let dr = self.r as i32 - other.r as i32;
        let dg = self.g as i32 - other.g as i32;
        let db = self.b as i32 - other.b as i32;

        dr * dr + dg * dg + db * db
    }

    #[inline(always)]
    fn mix_2_1_over_3_saturate(&self, other: &Self) -> Self {
        Self {
            r: mix_2_1_over_3_saturate(self.r, other.r),
            g: mix_2_1_over_3_saturate(self.g, other.g),
            b: mix_2_1_over_3_saturate(self.b, other.b),
            a: 0,
        }
    }

    #[inline(always)]
    fn mix_1_2_over_3_saturate(&self, other: &Self) -> Self {
        Self {
            r: mix_1_2_over_3_saturate(self.r, other.r),
            g: mix_1_2_over_3_saturate(self.g, other.g),
            b: mix_1_2_over_3_saturate(self.b, other.b),
            a: 0,
        }
    }

    #[inline(always)]
    fn mix_1_1_over_2_saturate(&self, other: &Self) -> Self {
        Self {
            r: mix_1_1_over_2_saturate(self.r, other.r),
            g: mix_1_1_over_2_saturate(self.g, other.g),
            b: mix_1_1_over_2_saturate(self.b, other.b),
            a: 0,
        }
    }

    fn contains_alpha() -> bool {
        true
    }

    fn alpha(&self) -> u8 {
        self.a
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Rgb8 {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb8 {
    #[inline(always)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// Implement Rgba for Rgb by faking alpha channel value.
impl ColorRgba8 for Rgb8 {
    #[inline(always)]
    fn to_565(&self) -> u16 {
        (((self.r & 0b11111000) as u16) << 8)
            + (((self.g & 0b11111100) as u16) << 3)
            + (self.b >> 3) as u16
    }

    #[inline(always)]
    fn luminance(&self) -> i32 {
        self.r as i32 + (self.g as i32) * 2 + self.b as i32
    }

    #[inline(always)]
    fn sqr_distance(&self, other: &Self) -> i32 {
        let dr = self.r as i32 - other.r as i32;
        let dg = self.g as i32 - other.g as i32;
        let db = self.b as i32 - other.b as i32;

        dr * dr + dg * dg + db * db
    }

    #[inline(always)]
    fn mix_2_1_over_3_saturate(&self, other: &Self) -> Self {
        Self {
            r: mix_2_1_over_3_saturate(self.r, other.r),
            g: mix_2_1_over_3_saturate(self.g, other.g),
            b: mix_2_1_over_3_saturate(self.b, other.b),
        }
    }

    #[inline(always)]
    fn mix_1_2_over_3_saturate(&self, other: &Self) -> Self {
        Self {
            r: mix_1_2_over_3_saturate(self.r, other.r),
            g: mix_1_2_over_3_saturate(self.g, other.g),
            b: mix_1_2_over_3_saturate(self.b, other.b),
        }
    }

    #[inline(always)]
    fn mix_1_1_over_2_saturate(&self, other: &Self) -> Self {
        Self {
            r: mix_1_1_over_2_saturate(self.r, other.r),
            g: mix_1_1_over_2_saturate(self.g, other.g),
            b: mix_1_1_over_2_saturate(self.b, other.b),
        }
    }

    fn contains_alpha() -> bool {
        false
    }

    fn alpha(&self) -> u8 {
        255
    }
}

pub fn min_max_luminance<T: ColorRgba8>(block: &[T]) -> (T, T) {
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

    (min, max)
}

pub trait ColorRed8: Copy + Clone + Default {
    fn red(&self) -> u8;
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Red8 {
    pub red: u8,
}

impl ColorRed8 for Red8 {
    fn red(&self) -> u8 {
        self.red
    }
}

pub trait ColorRedGreen8: Copy + Clone + Default {
    fn red(&self) -> u8;
    fn green(&self) -> u8;
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct RedGreen8 {
    pub red: u8,
    pub green: u8,
}

impl ColorRedGreen8 for RedGreen8 {
    fn red(&self) -> u8 {
        self.red
    }

    fn green(&self) -> u8 {
        self.green
    }
}
