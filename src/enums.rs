#![allow(non_upper_case_globals)]

/// Wraps an RGBA color
pub struct Color(pub u32);

impl Color {
    pub const IndianRed: Color = Color::rgb(205, 92, 92);
    pub const LightCoral: Color = Color::rgb(240, 128, 128);
    pub const Salmon: Color = Color::rgb(250, 128, 114);
    pub const DarkSalmon: Color = Color::rgb(233, 150, 122);
    pub const LightSalmon: Color = Color::rgb(255, 160, 122);
    pub const Crimson: Color = Color::rgb(220, 20, 60);
    pub const Red: Color = Color::rgb(255, 0, 0);
    pub const FireBrick: Color = Color::rgb(178, 34, 34);
    pub const DarkRed: Color = Color::rgb(139, 0, 0);
    pub const Pink: Color = Color::rgb(255, 192, 203);
    pub const LightPink: Color = Color::rgb(255, 182, 193);
    pub const HotPink: Color = Color::rgb(255, 105, 180);
    pub const DeepPink: Color = Color::rgb(255, 20, 147);
    pub const MediumVioletRed: Color = Color::rgb(199, 21, 133);
    pub const PaleVioletRed: Color = Color::rgb(219, 112, 147);
    pub const Coral: Color = Color::rgb(255, 127, 80);
    pub const Tomato: Color = Color::rgb(255, 99, 71);
    pub const OrangeRed: Color = Color::rgb(255, 69, 0);
    pub const DarkOrange: Color = Color::rgb(255, 140, 0);
    pub const Orange: Color = Color::rgb(255, 165, 0);
    pub const Gold: Color = Color::rgb(255, 215, 0);
    pub const Yellow: Color = Color::rgb(255, 255, 0);
    pub const LightYellow: Color = Color::rgb(255, 255, 224);
    pub const LemonChiffon: Color = Color::rgb(255, 250, 205);
    pub const LightGoldenrodYellow: Color = Color::rgb(250, 250, 210);
    pub const PapayaWhip: Color = Color::rgb(255, 239, 213);
    pub const Moccasin: Color = Color::rgb(255, 228, 181);
    pub const PeachPuff: Color = Color::rgb(255, 218, 185);
    pub const PaleGoldenrod: Color = Color::rgb(238, 232, 170);
    pub const Khaki: Color = Color::rgb(240, 230, 140);
    pub const DarkKhaki: Color = Color::rgb(189, 183, 107);
    pub const Lavender: Color = Color::rgb(230, 230, 250);
    pub const Thistle: Color = Color::rgb(216, 191, 216);
    pub const Plum: Color = Color::rgb(221, 160, 221);
    pub const Violet: Color = Color::rgb(238, 130, 238);
    pub const Orchid: Color = Color::rgb(218, 112, 214);
    pub const Fuchsia: Color = Color::rgb(255, 0, 255);
    pub const Magenta: Color = Color::rgb(255, 0, 255);
    pub const MediumOrchid: Color = Color::rgb(186, 85, 211);
    pub const MediumPurple: Color = Color::rgb(147, 112, 219);
    pub const RebeccaPurple: Color = Color::rgb(102, 51, 153);
    pub const BlueViolet: Color = Color::rgb(138, 43, 226);
    pub const DarkViolet: Color = Color::rgb(148, 0, 211);
    pub const DarkOrchid: Color = Color::rgb(153, 50, 204);
    pub const DarkMagenta: Color = Color::rgb(139, 0, 139);
    pub const Purple: Color = Color::rgb(128, 0, 128);
    pub const Indigo: Color = Color::rgb(75, 0, 130);
    pub const SlateBlue: Color = Color::rgb(106, 90, 205);
    pub const DarkSlateBlue: Color = Color::rgb(72, 61, 139);
    pub const MediumSlateBlue: Color = Color::rgb(123, 104, 238);
    pub const GreenYellow: Color = Color::rgb(173, 255, 47);
    pub const Chartreuse: Color = Color::rgb(127, 255, 0);
    pub const LawnGreen: Color = Color::rgb(124, 252, 0);
    pub const Lime: Color = Color::rgb(0, 255, 0);
    pub const LimeGreen: Color = Color::rgb(50, 205, 50);
    pub const PaleGreen: Color = Color::rgb(152, 251, 152);
    pub const LightGreen: Color = Color::rgb(144, 238, 144);
    pub const MediumSpringGreen: Color = Color::rgb(0, 250, 154);
    pub const SpringGreen: Color = Color::rgb(0, 255, 127);
    pub const MediumSeaGreen: Color = Color::rgb(60, 179, 113);
    pub const SeaGreen: Color = Color::rgb(46, 139, 87);
    pub const ForestGreen: Color = Color::rgb(34, 139, 34);
    pub const Green: Color = Color::rgb(0, 128, 0);
    pub const DarkGreen: Color = Color::rgb(0, 100, 0);
    pub const YellowGreen: Color = Color::rgb(154, 205, 50);
    pub const OliveDrab: Color = Color::rgb(107, 142, 35);
    pub const Olive: Color = Color::rgb(128, 128, 0);
    pub const DarkOliveGreen: Color = Color::rgb(85, 107, 47);
    pub const MediumAquamarine: Color = Color::rgb(102, 205, 170);
    pub const DarkSeaGreen: Color = Color::rgb(143, 188, 139);
    pub const LightSeaGreen: Color = Color::rgb(32, 178, 170);
    pub const DarkCyan: Color = Color::rgb(0, 139, 139);
    pub const Teal: Color = Color::rgb(0, 128, 128);
    pub const Aqua: Color = Color::rgb(0, 255, 255);
    pub const Cyan: Color = Color::rgb(0, 255, 255);
    pub const LightCyan: Color = Color::rgb(224, 255, 255);
    pub const PaleTurquoise: Color = Color::rgb(175, 238, 238);
    pub const Aquamarine: Color = Color::rgb(127, 255, 212);
    pub const Turquoise: Color = Color::rgb(64, 224, 208);
    pub const MediumTurquoise: Color = Color::rgb(72, 209, 204);
    pub const DarkTurquoise: Color = Color::rgb(0, 206, 209);
    pub const CadetBlue: Color = Color::rgb(95, 158, 160);
    pub const SteelBlue: Color = Color::rgb(70, 130, 180);
    pub const LightSteelBlue: Color = Color::rgb(176, 196, 222);
    pub const PowderBlue: Color = Color::rgb(176, 224, 230);
    pub const LightBlue: Color = Color::rgb(173, 216, 230);
    pub const SkyBlue: Color = Color::rgb(135, 206, 235);
    pub const LightSkyBlue: Color = Color::rgb(135, 206, 250);
    pub const DeepSkyBlue: Color = Color::rgb(0, 191, 255);
    pub const DodgerBlue: Color = Color::rgb(30, 144, 255);
    pub const CornflowerBlue: Color = Color::rgb(100, 149, 237);
    pub const RoyalBlue: Color = Color::rgb(65, 105, 225);
    pub const Blue: Color = Color::rgb(0, 0, 255);
    pub const MediumBlue: Color = Color::rgb(0, 0, 205);
    pub const DarkBlue: Color = Color::rgb(0, 0, 139);
    pub const Navy: Color = Color::rgb(0, 0, 128);
    pub const MidnightBlue: Color = Color::rgb(25, 25, 112);
    pub const Cornsilk: Color = Color::rgb(255, 248, 220);
    pub const BlanchedAlmond: Color = Color::rgb(255, 235, 205);
    pub const Bisque: Color = Color::rgb(255, 228, 196);
    pub const NavajoWhite: Color = Color::rgb(255, 222, 173);
    pub const Wheat: Color = Color::rgb(245, 222, 179);
    pub const BurlyWood: Color = Color::rgb(222, 184, 135);
    pub const Tan: Color = Color::rgb(210, 180, 140);
    pub const RosyBrown: Color = Color::rgb(188, 143, 143);
    pub const SandyBrown: Color = Color::rgb(244, 164, 96);
    pub const Goldenrod: Color = Color::rgb(218, 165, 32);
    pub const DarkGoldenrod: Color = Color::rgb(184, 134, 11);
    pub const Peru: Color = Color::rgb(205, 133, 63);
    pub const Chocolate: Color = Color::rgb(210, 105, 30);
    pub const SaddleBrown: Color = Color::rgb(139, 69, 19);
    pub const Sienna: Color = Color::rgb(160, 82, 45);
    pub const Brown: Color = Color::rgb(165, 42, 42);
    pub const Maroon: Color = Color::rgb(128, 0, 0);
    pub const White: Color = Color::rgb(255, 255, 255);
    pub const Snow: Color = Color::rgb(255, 250, 250);
    pub const HoneyDew: Color = Color::rgb(240, 255, 240);
    pub const MintCream: Color = Color::rgb(245, 255, 250);
    pub const Azure: Color = Color::rgb(240, 255, 255);
    pub const AliceBlue: Color = Color::rgb(240, 248, 255);
    pub const GhostWhite: Color = Color::rgb(248, 248, 255);
    pub const WhiteSmoke: Color = Color::rgb(245, 245, 245);
    pub const SeaShell: Color = Color::rgb(255, 245, 238);
    pub const Beige: Color = Color::rgb(245, 245, 220);
    pub const OldLace: Color = Color::rgb(253, 245, 230);
    pub const FloralWhite: Color = Color::rgb(255, 250, 240);
    pub const Ivory: Color = Color::rgb(255, 255, 240);
    pub const AntiqueWhite: Color = Color::rgb(250, 235, 215);
    pub const Linen: Color = Color::rgb(250, 240, 230);
    pub const LavenderBlush: Color = Color::rgb(255, 240, 245);
    pub const MistyRose: Color = Color::rgb(255, 228, 225);
    pub const Gainsboro: Color = Color::rgb(220, 220, 220);
    pub const LightGray: Color = Color::rgb(211, 211, 211);
    pub const Silver: Color = Color::rgb(192, 192, 192);
    pub const DarkGray: Color = Color::rgb(169, 169, 169);
    pub const Gray: Color = Color::rgb(128, 128, 128);
    pub const DimGray: Color = Color::rgb(105, 105, 105);
    pub const LightSlateGray: Color = Color::rgb(119, 136, 153);
    pub const SlateGray: Color = Color::rgb(112, 128, 144);
    pub const DarkSlateGray: Color = Color::rgb(47, 79, 79);
    pub const Black: Color = Color::rgb(0, 0, 0);
    /// Constructs a color from rgb values
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }
    /// Constructs a color from rgba values
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(
            ((r as u32 & 0xff) << 24)
                + ((g as u32 & 0xff) << 16)
                + ((b as u32 & 0xff) << 8)
                + (a as u32 & 0xff),
        )
    }
}
