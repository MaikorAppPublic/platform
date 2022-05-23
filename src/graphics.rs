pub mod sprite {
    pub mod byte3 {
        pub const MASK_ID: u8 = 0b11111111;
    }

    pub mod byte4 {
        pub const MASK_FLIP_V: u8 = 0b10000000;
        pub const MASK_FLIP_H: u8 = 0b01000000;
        pub const MASK_HALF_ALPHA: u8 = 0b00100000;
        pub const MASK_ROTATED: u8 = 0b00010000;
        pub const MASK_ENABLED: u8 = 0b00000001;
    }

    pub mod byte5 {
        pub const MASK_LARGE: u8 = 0b10000000;
        pub const MASK_ORDER: u8 = 0b01100000;
        pub const MASK_ATLAS: u8 = 0b00011000;
        pub const MASK_PALETTE: u8 = 0b00000011;
        pub const OFFSET_ATLAS: usize = 3;
        pub const OFFSET_ORDER: usize = 5;
    }
}

pub mod layer_header {
    pub mod byte3 {
        pub const MASK_ENABLED: u8 = 0b00000001;
        pub const MASK_ATLAS: u8 = 0b00000110;
        pub const MASK_ORDER: u8 = 0b00011000;
        pub const OFFSET_ATLAS: usize = 1;
        pub const OFFSET_ORDER: usize = 3;
    }
}

pub mod layer_tile {
    pub mod byte2 {
        pub const MASK_FLIP_V: u8 = 0b10000000;
        pub const MASK_FLIP_H: u8 = 0b01000000;
        pub const MASK_PALETTE: u8 = 0b00110000;
        pub const MASK_HALF_ALPHA: u8 = 0b00001000;
        pub const MASK_ROTATED: u8 = 0b00000100;
        pub const OFFSET_PALETTE: usize = 4;
    }
}
