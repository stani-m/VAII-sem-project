#[derive(Copy, Clone, Default, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Color(u8, u8, u8);

#[allow(dead_code)]
impl Color {
    // Basic colors
    pub const BLACK: Self = Self(0, 0, 0);
    pub const WHITE: Self = Self(255, 255, 255);
    pub const RED: Self = Self(255, 0, 0);
    pub const GREEN: Self = Self(0, 255, 0);
    pub const BLUE: Self = Self(0, 0, 255);
    pub const CYAN: Self = Self(0, 255, 255);
    pub const MAGENTA: Self = Self(255, 0, 255);
    pub const YELLOW: Self = Self(255, 255, 0);

    // Pink colors
    pub const MEDIUM_VIOLET_RED: Self = Self(199, 21, 133);
    pub const DEEP_PINK: Self = Self(255, 20, 147);
    pub const PALE_VIOLET_RED: Self = Self(219, 112, 147);
    pub const HOT_PINK: Self = Self(255, 105, 180);
    pub const LIGHT_PINK: Self = Self(255, 182, 193);
    pub const PINK: Self = Self(255, 192, 203);

    // Red colors
    pub const DARK_RED: Self = Self(139, 0, 0);
    pub const FIREBRICK: Self = Self(178, 34, 34);
    pub const CRIMSON: Self = Self(220, 20, 60);
    pub const INDIAN_RED: Self = Self(205, 92, 92);
    pub const LIGHT_CORAL: Self = Self(240, 128, 128);
    pub const SALMON: Self = Self(250, 128, 114);
    pub const DARK_SALMON: Self = Self(233, 150, 122);
    pub const LIGHT_SALMON: Self = Self(255, 160, 122);

    // Orange colors
    pub const ORANGE_RED: Self = Self(255, 69, 0);
    pub const TOMATO: Self = Self(255, 99, 71);
    pub const DARK_ORANGE: Self = Self(255, 140, 0);
    pub const CORAL: Self = Self(255, 127, 80);
    pub const ORANGE: Self = Self(255, 165, 0);

    // Yellow colors
    pub const DARK_KHAKI: Self = Self(189, 183, 107);
    pub const GOLD: Self = Self(255, 215, 0);
    pub const KHAKI: Self = Self(240, 230, 140);
    pub const PEACH_PUFF: Self = Self(255, 218, 185);
    pub const PALE_GOLDENROD: Self = Self(238, 232, 170);
    pub const MOCCASIN: Self = Self(255, 228, 181);
    pub const PAPAYA_WHIP: Self = Self(255, 239, 213);
    pub const LIGHT_GOLDENROD_YELLOW: Self = Self(250, 250, 210);
    pub const LEMON_CHIFFON: Self = Self(255, 250, 205);
    pub const LIGHT_YELLOW: Self = Self(255, 255, 224);

    // Brown colors
    pub const MAROON: Self = Self(128, 0, 0);
    pub const BROWN: Self = Self(165, 42, 42);
    pub const SADDLE_BROWN: Self = Self(139, 69, 19);
    pub const SIENNA: Self = Self(160, 82, 45);
    pub const CHOCOLATE: Self = Self(210, 105, 30);
    pub const DARK_GOLDENROD: Self = Self(184, 134, 11);
    pub const PERU: Self = Self(205, 133, 63);
    pub const ROSY_BROWN: Self = Self(188, 143, 143);
    pub const GOLDENROD: Self = Self(218, 165, 32);
    pub const SANDY_BROWN: Self = Self(244, 164, 96);
    pub const TAN: Self = Self(210, 180, 140);
    pub const BURLYWOOD: Self = Self(222, 184, 135);
    pub const WHEAT: Self = Self(245, 222, 179);
    pub const NAVAJO_WHITE: Self = Self(255, 222, 173);
    pub const BISQUE: Self = Self(255, 228, 196);
    pub const BLANCHED_ALMOND: Self = Self(255, 235, 205);
    pub const CORNSILK: Self = Self(255, 248, 220);

    // Green colors
    pub const DARK_GREEN: Self = Self(0, 100, 0);
    pub const DARK_OLIVE_GREEN: Self = Self(85, 107, 47);
    pub const FOREST_GREEN: Self = Self(34, 139, 34);
    pub const SEA_GREEN: Self = Self(46, 139, 87);
    pub const OLIVE: Self = Self(128, 128, 0);
    pub const OLIVE_DRAB: Self = Self(107, 142, 35);
    pub const MEDIUM_SEA_GREEN: Self = Self(60, 179, 113);
    pub const LIME_GREEN: Self = Self(50, 205, 50);
    pub const LIME: Self = Self(0, 255, 0);
    pub const SPRING_GREEN: Self = Self(0, 255, 127);
    pub const MEDIUM_SPRING_GREEN: Self = Self(0, 250, 154);
    pub const DARK_SEA_GREEN: Self = Self(143, 188, 143);
    pub const MEDIUM_AQUAMARINE: Self = Self(102, 205, 170);
    pub const YELLOW_GREEN: Self = Self(154, 205, 50);
    pub const LAWN_GREEN: Self = Self(124, 252, 0);
    pub const CHARTREUSE: Self = Self(127, 255, 0);
    pub const LIGHT_GREEN: Self = Self(144, 238, 144);
    pub const GREEN_YELLOW: Self = Self(173, 255, 47);
    pub const PALE_GREEN: Self = Self(152, 251, 152);

    // Cyan colors
    pub const TEAL: Self = Self(0, 128, 128);
    pub const DARK_CYAN: Self = Self(0, 139, 139);
    pub const LIGHT_SEA_GREEN: Self = Self(32, 178, 170);
    pub const CADET_BLUE: Self = Self(95, 158, 160);
    pub const DARK_TURQUOISE: Self = Self(0, 206, 209);
    pub const MEDIUM_TURQUOISE: Self = Self(72, 209, 204);
    pub const TURQUOISE: Self = Self(64, 224, 208);
    pub const AQUA: Self = Self(0, 255, 255);
    pub const AQUAMARINE: Self = Self(127, 255, 212);
    pub const PALE_TURQUOISE: Self = Self(175, 238, 238);
    pub const LIGHT_CYAN: Self = Self(224, 255, 255);

    // Blue colors
    pub const NAVY: Self = Self(0, 0, 128);
    pub const DARK_BLUE: Self = Self(0, 0, 139);
    pub const MEDIUM_BLUE: Self = Self(0, 0, 205);
    pub const MIDNIGHT_BLUE: Self = Self(25, 25, 112);
    pub const ROYAL_BLUE: Self = Self(65, 105, 225);
    pub const STEEL_BLUE: Self = Self(70, 130, 180);
    pub const DODGER_BLUE: Self = Self(30, 144, 255);
    pub const DEEP_SKY_BLUE: Self = Self(0, 191, 255);
    pub const CORNFLOWER_BLUE: Self = Self(100, 149, 237);
    pub const SKY_BLUE: Self = Self(135, 206, 235);
    pub const LIGHT_SKY_BLUE: Self = Self(135, 206, 250);
    pub const LIGHT_STEEL_BLUE: Self = Self(176, 196, 222);
    pub const LIGHT_BLUE: Self = Self(173, 216, 230);
    pub const POWDER_BLUE: Self = Self(176, 224, 230);

    // Purple, violet, and magenta colors
    pub const INDIGO: Self = Self(75, 0, 130);
    pub const DARK_MAGENTA: Self = Self(139, 0, 139);
    pub const DARK_VIOLET: Self = Self(148, 0, 211);
    pub const DARK_SLATE_BLUE: Self = Self(72, 61, 139);
    pub const BLUE_VIOLET: Self = Self(138, 43, 226);
    pub const DARK_ORCHID: Self = Self(153, 50, 204);
    pub const FUCHSIA: Self = Self(255, 0, 255);
    pub const SLATE_BLUE: Self = Self(106, 90, 205);
    pub const MEDIUM_SLATE_BLUE: Self = Self(123, 104, 238);
    pub const MEDIUM_ORCHID: Self = Self(186, 85, 211);
    pub const MEDIUM_PURPLE: Self = Self(147, 112, 219);
    pub const ORCHID: Self = Self(218, 112, 214);
    pub const VIOLET: Self = Self(238, 130, 238);
    pub const PLUM: Self = Self(221, 160, 221);
    pub const THISTLE: Self = Self(216, 191, 216);
    pub const LAVENDER: Self = Self(230, 230, 250);

    // White colors
    pub const MISTY_ROSE: Self = Self(255, 228, 225);
    pub const ANTIQUE_WHITE: Self = Self(250, 235, 215);
    pub const LINEN: Self = Self(250, 240, 230);
    pub const BEIGE: Self = Self(245, 245, 220);
    pub const WHITE_SMOKE: Self = Self(245, 245, 245);
    pub const LAVENDER_BLUSH: Self = Self(255, 240, 245);
    pub const OLD_LACE: Self = Self(253, 245, 230);
    pub const ALICE_BLUE: Self = Self(240, 248, 255);
    pub const SEASHELL: Self = Self(255, 245, 238);
    pub const GHOST_WHITE: Self = Self(248, 248, 255);
    pub const HONEYDEW: Self = Self(240, 255, 240);
    pub const FLORAL_WHITE: Self = Self(255, 250, 240);
    pub const AZURE: Self = Self(240, 255, 255);
    pub const MINT_CREAM: Self = Self(245, 255, 250);
    pub const SNOW: Self = Self(255, 250, 250);
    pub const IVORY: Self = Self(255, 255, 240);

    // Gray and black colors
    pub const DARK_SLATE_GRAY: Self = Self(47, 79, 79);
    pub const DIM_GRAY: Self = Self(105, 105, 105);
    pub const SLATE_GRAY: Self = Self(112, 128, 144);
    pub const GRAY: Self = Self(128, 128, 128);
    pub const LIGHT_SLATE_GRAY: Self = Self(119, 136, 153);
    pub const DARK_GRAY: Self = Self(169, 169, 169);
    pub const SILVER: Self = Self(192, 192, 192);
    pub const LIGHT_GRAY: Self = Self(211, 211, 211);
    pub const GAINSBORO: Self = Self(220, 220, 220);
}
