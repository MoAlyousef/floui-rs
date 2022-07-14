#![allow(non_upper_case_globals)]

/// Wraps an RGBA color
pub struct Color(pub u32);

impl Color {
    /// White color
    pub const White: Color = Color(0xffffffff);
    /// Red color
    pub const Red: Color = Color(0xff0000ff);
    /// Green color
    pub const Green: Color = Color(0x00ff00ff);
    /// Blue color
    pub const Blue: Color = Color(0x0000ffff);
    /// Black color
    pub const Black: Color = Color(0x000000ff);
    /// Yellow color
    pub const Yellow: Color = Color(0xffff00ff);
    /// Orange color
    pub const Orange: Color = Color(0xff7f00ff);
    /// LightGray color
    pub const LightGray: Color = Color(0xaaaaaaff);
    /// Gray color
    pub const Gray: Color = Color(0x7f7f7fff);
    /// DarkGray color
    pub const DarkGray: Color = Color(0x555555ff);
    /// Magenta color
    pub const Magenta: Color = Color(0xff00ffff);
    /// Constructs a color from rgb values
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }
    /// Constructs a color from rgba values
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(
            ((r as u32 & 0xff) << 24)
                + ((g as u32 & 0xff) << 16)
                + ((b as u32 & 0xff) << 8)
                + (a as u32 & 0xff),
        )
    }
}
