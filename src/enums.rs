#![allow(non_upper_case_globals)]

pub struct Color(pub u32);

impl Color {
    pub const White: Color = Color(0xffffffff);
    pub const Red: Color = Color(0xff0000ff);
    pub const Green: Color = Color(0x00ff00ff);
    pub const Blue: Color = Color(0x0000ffff);
    pub const Black: Color = Color(0x000000ff);
    pub const Yellow: Color = Color(0xffff00ff);
    pub const Orange: Color = Color(0xff7f00ff);
    pub const LightGray: Color = Color(0xaaaaaaff);
    pub const Gray: Color = Color(0x7f7f7fff);
    pub const DarkGray: Color = Color(0x555555ff);
    pub const Magenta: Color = Color(0xff00ffff);
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(((r as u32 & 0xff) << 24) + ((g as u32 & 0xff) << 16) + ((b as u32 & 0xff) << 8) + (a as u32 & 0xff))
    }
}