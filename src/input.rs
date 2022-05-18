/// Values to check against when a button is pressed
pub mod mask {
    /// Bit mask for mem::INPUT
    pub const UP: usize = 0b00000001;
    pub const DOWN: usize = 0b00000010;
    pub const LEFT: usize = 0b00000100;
    pub const RIGHT: usize = 0b00001000;

    /// Bit mask for mem::INPUT+1
    pub const A: usize = 0b00000001;
    pub const B: usize = 0b00000010;
    pub const START: usize = 0b00000100;
    pub const L: usize = 0b00001000;
    pub const R: usize = 0b00010000;
    pub const X: usize = 0b00100000;
    pub const Y: usize = 0b01000000;
}

/// Type of the active controller
/// Controlled by host
/// Controls CONTROLLER_GRAPHICS bank
pub mod controller_type {
    pub const UNKNOWN: u8 = 0;
    pub const XBOX: u8 = 1;
    pub const PLAYSTATION: u8 = 2;
    pub const SWITCH: u8 = 3;
    pub const SCREEN: u8 = 4;
    pub const KEYBOARD1: u8 = 5;
    pub const KEYBOARD2: u8 = 6;
    pub const KEYBOARD3: u8 = 7;
    pub const KEYBOARD4: u8 = 8;

    pub const COUNT: usize = 9;
}

/// Address of graphics in memory
pub mod input_address {
    use crate::mem::{address, sizes};

    pub const UP: u16 = address::CONTROLLER_GRAPHICS;
    pub const DOWN: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC;
    pub const LEFT: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 2;
    pub const RIGHT: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 3;
    pub const START: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 4;
    pub const A: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 5;
    pub const B: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 6;
    pub const X: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 7;
    pub const Y: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 8;
    pub const L: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 9;
    pub const R: u16 = address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHIC * 10;
}

/// Mask for controller sprite control byte
pub mod control {
    pub const ENABLED: u8 = 0b10000000;
    pub const ID: u8 = 0b01111000;
}

/// Button ID for controller sprite control byte
pub mod id {
    pub const UP: u8 = 0b00000000; //0, x0
    pub const DOWN: u8 = 0b00001000; //8, x08
    pub const LEFT: u8 = 0b00010000; //16, x10
    pub const RIGHT: u8 = 0b00011000; //24, x18
    pub const START: u8 = 0b00100000; //32, x20
    pub const A: u8 = 0b01000000; //64, x40
    pub const B: u8 = 0b01001000; //72, x48
    pub const X: u8 = 0b01010000; //80, x50
    pub const Y: u8 = 0b01011000; //88, x58
    pub const L: u8 = 0b01100000; //96, x60
    pub const R: u8 = 0b01101000; //104, x68
}
