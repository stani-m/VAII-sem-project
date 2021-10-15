#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Color(u8, u8, u8);

#[allow(dead_code)]
impl Color {
    // Basic colors
    pub const BLACK: Self = Color(0, 0, 0);
    pub const WHITE: Self = Color(255, 255, 255);
    pub const RED: Self = Color(255, 0, 0);
    pub const GREEN: Self = Color(0, 255, 0);
    pub const BLUE: Self = Color(0, 0, 255);
    pub const CYAN: Self = Color(0, 255, 255);
    pub const MAGENTA: Self = Color(255, 0, 255);
    pub const YELLOW: Self = Color(255, 255, 0);

    // Pink colors
    pub const MEDIUM_VIOLET_RED: Self = Color(199, 21, 133);
    pub const DEEP_PINK: Self = Color(255, 20, 147);
    pub const PALE_VIOLET_RED: Self = Color(219, 112, 147);
    pub const HOT_PINK: Self = Color(255, 105, 180);
    pub const LIGHT_PINK: Self = Color(255, 182, 193);
    pub const PINK: Self = Color(255, 192, 203);

    // Red colors
    pub const DARK_RED: Self = Color(139, 0, 0);
    pub const FIREBRICK: Self = Color(178, 34, 34);
    pub const CRIMSON: Self = Color(220, 20, 60);
    pub const INDIAN_RED: Self = Color(205, 92, 92);
    pub const LIGHT_CORAL: Self = Color(240, 128, 128);
    pub const SALMON: Self = Color(250, 128, 114);
    pub const DARK_SALMON: Self = Color(233, 150, 122);
    pub const LIGHT_SALMON: Self = Color(255, 160, 122);

    // Orange colors
    pub const ORANGE_RED: Self = Color(255, 69, 0);
    pub const TOMATO: Self = Color(255, 99, 71);
    pub const DARK_ORANGE: Self = Color(255, 140, 0);
    pub const CORAL: Self = Color(255, 127, 80);
    pub const ORANGE: Self = Color(255, 165, 0);

    // Yellow colors
    pub const DARK_KHAKI: Self = Color(189, 183, 107);
    pub const GOLD: Self = Color(255, 215, 0);
    pub const KHAKI: Self = Color(240, 230, 140);
    pub const PEACH_PUFF: Self = Color(255, 218, 185);
    pub const PALE_GOLDENROD: Self = Color(238, 232, 170);
    pub const MOCCASIN: Self = Color(255, 228, 181);
    pub const PAPAYA_WHIP: Self = Color(255, 239, 213);
    pub const LIGHT_GOLDENROD_YELLOW: Self = Color(250, 250, 210);
    pub const LEMON_CHIFFON: Self = Color(255, 250, 205);
    pub const LIGHT_YELLOW: Self = Color(255, 255, 224);

    // Brown colors
    pub const MAROON: Self = Color(128, 0, 0);
    pub const BROWN: Self = Color(165, 42, 42);
    pub const SADDLE_BROWN: Self = Color(139, 69, 19);
    pub const SIENNA: Self = Color(160, 82, 45);
    pub const CHOCOLATE: Self = Color(210, 105, 30);
    pub const DARK_GOLDENROD: Self = Color(184, 134, 11);
    pub const PERU: Self = Color(205, 133, 63);
    pub const ROSY_BROWN: Self = Color(188, 143, 143);
    pub const GOLDENROD: Self = Color(218, 165, 32);
    pub const SANDY_BROWN: Self = Color(244, 164, 96);
    pub const TAN: Self = Color(210, 180, 140);
    pub const BURLYWOOD: Self = Color(222, 184, 135);
    pub const WHEAT: Self = Color(245, 222, 179);
    pub const NAVAJO_WHITE: Self = Color(255, 222, 173);
    pub const BISQUE: Self = Color(255, 228, 196);
    pub const BLANCHED_ALMOND: Self = Color(255, 235, 205);
    pub const CORNSILK: Self = Color(255, 248, 220);

    // Green colors
    pub const DARK_GREEN: Self = Color(0, 100, 0);
    pub const DARK_OLIVE_GREEN: Self = Color(85, 107, 47);
    pub const FOREST_GREEN: Self = Color(34, 139, 34);
    pub const SEA_GREEN: Self = Color(46, 139, 87);
    pub const OLIVE: Self = Color(128, 128, 0);
    pub const OLIVE_DRAB: Self = Color(107, 142, 35);
    pub const MEDIUM_SEA_GREEN: Self = Color(60, 179, 113);
    pub const LIME_GREEN: Self = Color(50, 205, 50);
    pub const LIME: Self = Color(0, 255, 0);
    pub const SPRING_GREEN: Self = Color(0, 255, 127);
    pub const MEDIUM_SPRING_GREEN: Self = Color(0, 250, 154);
    pub const DARK_SEA_GREEN: Self = Color(143, 188, 143);
    pub const MEDIUM_AQUAMARINE: Self = Color(102, 205, 170);
    pub const YELLOW_GREEN: Self = Color(154, 205, 50);
    pub const LAWN_GREEN: Self = Color(124, 252, 0);
    pub const CHARTREUSE: Self = Color(127, 255, 0);
    pub const LIGHT_GREEN: Self = Color(144, 238, 144);
    pub const GREEN_YELLOW: Self = Color(173, 255, 47);
    pub const PALE_GREEN: Self = Color(152, 251, 152);

    // Cyan colors
    pub const TEAL: Self = Color(0, 128, 128);
    pub const DARK_CYAN: Self = Color(0, 139, 139);
    pub const LIGHT_SEA_GREEN: Self = Color(32, 178, 170);
    pub const CADET_BLUE: Self = Color(95, 158, 160);
    pub const DARK_TURQUOISE: Self = Color(0, 206, 209);
    pub const MEDIUM_TURQUOISE: Self = Color(72, 209, 204);
    pub const TURQUOISE: Self = Color(64, 224, 208);
    pub const AQUA: Self = Color(0, 255, 255);
    pub const AQUAMARINE: Self = Color(127, 255, 212);
    pub const PALE_TURQUOISE: Self = Color(175, 238, 238);
    pub const LIGHT_CYAN: Self = Color(224, 255, 255);

    // Blue colors
    pub const NAVY: Self = Color(0, 0, 128);
    pub const DARK_BLUE: Self = Color(0, 0, 139);
    pub const MEDIUM_BLUE: Self = Color(0, 0, 205);
    pub const MIDNIGHT_BLUE: Self = Color(25, 25, 112);
    pub const ROYAL_BLUE: Self = Color(65, 105, 225);
    pub const STEEL_BLUE: Self = Color(70, 130, 180);
    pub const DODGER_BLUE: Self = Color(30, 144, 255);
    pub const DEEP_SKY_BLUE: Self = Color(0, 191, 255);
    pub const CORNFLOWER_BLUE: Self = Color(100, 149, 237);
    pub const SKY_BLUE: Self = Color(135, 206, 235);
    pub const LIGHT_SKY_BLUE: Self = Color(135, 206, 250);
    pub const LIGHT_STEEL_BLUE: Self = Color(176, 196, 222);
    pub const LIGHT_BLUE: Self = Color(173, 216, 230);
    pub const POWDER_BLUE: Self = Color(176, 224, 230);

    // Purple, violet, and magenta colors
    pub const INDIGO: Self = Color(75, 0, 130);
    pub const DARK_MAGENTA: Self = Color(139, 0, 139);
    pub const DARK_VIOLET: Self = Color(148, 0, 211);
    pub const DARK_SLATE_BLUE: Self = Color(72, 61, 139);
    pub const BLUE_VIOLET: Self = Color(138, 43, 226);
    pub const DARK_ORCHID: Self = Color(153, 50, 204);
    pub const FUCHSIA: Self = Color(255, 0, 255);
    pub const SLATE_BLUE: Self = Color(106, 90, 205);
    pub const MEDIUM_SLATE_BLUE: Self = Color(123, 104, 238);
    pub const MEDIUM_ORCHID: Self = Color(186, 85, 211);
    pub const MEDIUM_PURPLE: Self = Color(147, 112, 219);
    pub const ORCHID: Self = Color(218, 112, 214);
    pub const VIOLET: Self = Color(238, 130, 238);
    pub const PLUM: Self = Color(221, 160, 221);
    pub const THISTLE: Self = Color(216, 191, 216);
    pub const LAVENDER: Self = Color(230, 230, 250);

    // White colors
    pub const MISTY_ROSE: Self = Color(255, 228, 225);
    pub const ANTIQUE_WHITE: Self = Color(250, 235, 215);
    pub const LINEN: Self = Color(250, 240, 230);
    pub const BEIGE: Self = Color(245, 245, 220);
    pub const WHITE_SMOKE: Self = Color(245, 245, 245);
    pub const LAVENDER_BLUSH: Self = Color(255, 240, 245);
    pub const OLD_LACE: Self = Color(253, 245, 230);
    pub const ALICE_BLUE: Self = Color(240, 248, 255);
    pub const SEASHELL: Self = Color(255, 245, 238);
    pub const GHOST_WHITE: Self = Color(248, 248, 255);
    pub const HONEYDEW: Self = Color(240, 255, 240);
    pub const FLORAL_WHITE: Self = Color(255, 250, 240);
    pub const AZURE: Self = Color(240, 255, 255);
    pub const MINT_CREAM: Self = Color(245, 255, 250);
    pub const SNOW: Self = Color(255, 250, 250);
    pub const IVORY: Self = Color(255, 255, 240);

    // Gray and black colors
    pub const DARK_SLATE_GRAY: Self = Color(47, 79, 79);
    pub const DIM_GRAY: Self = Color(105, 105, 105);
    pub const SLATE_GRAY: Self = Color(112, 128, 144);
    pub const GRAY: Self = Color(128, 128, 128);
    pub const LIGHT_SLATE_GRAY: Self = Color(119, 136, 153);
    pub const DARK_GRAY: Self = Color(169, 169, 169);
    pub const SILVER: Self = Color(192, 192, 192);
    pub const LIGHT_GRAY: Self = Color(211, 211, 211);
    pub const GAINSBORO: Self = Color(220, 220, 220);
}
