pub mod sizes {
    use crate::constants::{LAYER_COUNT, PALETTE_COUNT, SPRITE_COUNT};

    pub const CODE_BANK: u16 = 8700;
    pub const RAM_BANK: u16 = 8700;
    pub const CODE: u16 = CODE_BANK * 2;
    pub const RAM: u16 = RAM_BANK * 2;
    pub const SOUND: u16 = 30;
    pub const WAVE_TABLE: u16 = 16;
    //Byte 0 is direction, byte 1 is action
    pub const INPUT: u16 = 2;
    pub const CODE_BANK_ID: u16 = 1;
    pub const RAM_BANK_ID: u16 = 1;
    pub const SAVE_BANK_ID: u16 = 1;
    pub const SAVE_BANK: u16 = 4096;
    pub const SAVE_CONTROL: u16 = 1;
    pub const SPRITE: u16 = 5;
    //255 sprites, each taking 5 bytes
    //(8b X, 8b Y, 8b tile id, 1b flipV, 1b flipH, 2b palette, 1b src size, 2b order, 1b half alpha, 1b rotated, 1b enabled, 2b atlas, 3b ?)
    pub const SPRITE_TABLE: u16 = SPRITE_COUNT as u16 * SPRITE;
    //4 layers, each header is made of 3 bytes (8b X, 8b Y, 1b visible, 2b atlas)
    pub const LAYERS_HEADER: u16 = 3 * LAYER_COUNT as u16;
    //4 layers, each made of 1320 (44x30) tiles, each made of 2 bytes (8b tile id, 1b flipV, 1b flipH, 2b palette, 1b half alpha, 1b rotated, 2b ?)
    pub const LAYERS_CONTENT: u16 = 1320 * 2 * LAYER_COUNT as u16;
    //4 palettes, each made of 16 colors, each color is 3 bytes
    pub const PALETTE: u16 = 16 * 3; //0,0,0 is transparent
    pub const PALETTES_TOTAL: u16 = PALETTE * PALETTE_COUNT as u16;
    //25x10 (tiles) 200x160 (pixels) 100x160 (bytes) atlas of palette index (two colour IDs per byte)
    pub const ATLAS: u16 = 100 * 40;
    pub const ATLAS_BANK_ID: u16 = 1;
    pub const STACK: u16 = 1000;
    pub const SP: u16 = 2;
    pub const FP: u16 = 2;
    pub const TIMER_CONTROL: u16 = 2;
    pub const TIMER_VALUE: u16 = 1;
    pub const TIMERS: u16 = TIMER_CONTROL + TIMER_VALUE * 4;
    pub const VLINE: u16 = 1;
    pub const IRQ_RET_ADDR: u16 = 2;
    pub const IRQ_REG_DUMP: u16 = 8;
    pub const IRQ_CONTROL: u16 = 1;
    pub const IRQ_INTERNAL: u16 = IRQ_RET_ADDR + IRQ_REG_DUMP + IRQ_CONTROL;
    //year (0=2000), month (1 = jan), day, hour (24), min, sec
    pub const DATETIME: u16 = 6;
    pub const RAND: u16 = 1;
    pub const CONTROLLER_TYPE: u16 = 1;
    //8 bytes per sprite (4 pixels per byte)
    pub const CONTROLLER_GRAPHIC: u16 = 8;
    pub const CONTROLLER_GRAPHICS: u16 = 11 * CONTROLLER_GRAPHIC;
    pub const CONTROLLER_PALETTE: u16 = 3 * 4; //0,0,0 is transparent
                                               //8 sprites, each taking 3 bytes
                                               //(bits) 8 x, 8 y, 1 enabled, 4 id, 3 ?
    pub const CONTROLLER_TABLE: u16 = 3 * 8;
    pub const CONTROLLER_TOTAL: u16 =
        CONTROLLER_TYPE + CONTROLLER_GRAPHICS + CONTROLLER_PALETTE + CONTROLLER_TABLE;
    pub const ATLAS_TOTAL: u16 = (ATLAS + ATLAS_BANK_ID) * 4;
    pub const LAYER_TOTAL: u16 = LAYERS_CONTENT + LAYERS_HEADER;
    pub const GRAPHICS_TOTAL: u16 =
        LAYER_TOTAL + SPRITE_TABLE + PALETTES_TOTAL + ATLAS_TOTAL + CONTROLLER_TOTAL;
    pub const SYSTEM_TOTAL: u16 =
        CODE + RAM + CODE_BANK_ID + RAM_BANK_ID + STACK + SP + FP + TIMERS + IRQ_INTERNAL + VLINE;
    pub const HARDWARE_TOTAL: u16 = WAVE_TABLE+
        SOUND + INPUT + SAVE_BANK_ID + SAVE_BANK + SAVE_CONTROL + DATETIME + RAND;
    pub const RESERVED: u16 = 34;
    pub const TOTAL: usize =
        (GRAPHICS_TOTAL + SYSTEM_TOTAL + HARDWARE_TOTAL) as usize + RESERVED as usize;
}

pub mod address {
    pub const CODE: u16 = 0x0; //0
    pub const CODE_BANK: u16 = 0x21FC; //8700
    pub const RAM: u16 = 0x43F8; //17400
    pub const RAM_BANK: u16 = 0x65F4; //26100
    pub const INPUT: u16 = 0x87F0; //34800
    pub const SOUND: u16 = 0x87F2; //34802
    pub const SAVE_BANK_ID: u16 = 0x8810; //34832
    pub const SAVE_BANK: u16 = 0x8811; //34833
    pub const ATLAS1: u16 = 0x9811; //38929
    pub const ATLAS2: u16 = 0xA7B1; //42929
    pub const ATLAS3: u16 = 0xB751; //46929
    pub const ATLAS4: u16 = 0xC6F1; //50929
    pub const PALETTES: u16 = 0xD691; //54929
    pub const SPRITE_TABLE: u16 = 0xD751; //55121
    pub const LAYER_HEADERS: u16 = 0xDC4C; //56396
    pub const LAYERS: u16 = 0xDC55; //56405
    pub const CODE_BANK_ID: u16 = 0xFB45; //64325
    pub const RAM_BANK_ID: u16 = 0xFB46; //64326
    pub const ATLAS1_BANK_ID: u16 = 0xFB47; //64327
    pub const ATLAS2_BANK_ID: u16 = 0xFB48; //64328
    pub const ATLAS3_BANK_ID: u16 = 0xFB49; //64329
    pub const ATLAS4_BANK_ID: u16 = 0xFB4A; //64330
    pub const SP: u16 = 0xFB4B; //64331
    pub const FP: u16 = 0xFB4D; //64333
    pub const TIMER_CONTROL: u16 = 0xFB4F; //64335
    pub const TIMER_VALUE1: u16 = 0xFB51; //64337
    pub const TIMER_VALUE2: u16 = 0xFB52; //64338
    pub const TIMER_VALUE3: u16 = 0xFB53; //64339
    pub const TIMER_VALUE4: u16 = 0xFB54; //64340
    pub const IRQ_RET_ADDR: u16 = 0xFB55; //64341
    pub const IRQ_REG_DUMP: u16 = 0xFB57; //64343
    pub const VLINE: u16 = 0xFB5F; //64351
    pub const CONTROLLER_TYPE: u16 = 0xFB60; //64352
    pub const CONTROLLER_GRAPHICS: u16 = 0xFB61; //64353
    pub const CONTROLLER_PALETTE: u16 = 0xFBB9; //64441
    pub const CONTROLLER_TABLE: u16 = 0xFBC5; //64453
    pub const IRQ_CONTROL: u16 = 0xFBDD; //64477
    pub const SAVE_CONTROL: u16 = 0xFBDE; //64478
    pub const DATETIME: u16 = 0xFBDF; //64479
    pub const RAND: u16 = 0xFBE5; //64485
    pub const WAVE_TABLE: u16 = 0xFBE6; //64486
    pub const RESERVED: u16 = 0xFBF6; //64502
    pub const STACK: u16 = 0xFC18; //64536
    pub const MAX: u16 = 0xFFFF; //65535

    pub mod interrupt {
        pub const IRQ_INPUT: u16 = 0x0500;
        pub const IRQ_LINE_DRAW: u16 = 0x0540;
        pub const IRQ_SCREEN_DRAW: u16 = 0x0800;
        pub const IRQ_TIMER: u16 = 0x05C0;
        pub const IRQ_CONTROLLER: u16 = 0x0600;
        pub const IRQ_DATETIME: u16 = 0x0640;
    }

    /// Changing values at 'special' addresses can take many more cycles than normal
    pub const fn is_special_memory(address: u16) -> bool {
        matches!(
            address,
            CODE_BANK_ID
                | RAM_BANK_ID
                | ATLAS1_BANK_ID
                | ATLAS2_BANK_ID
                | TIMER_CONTROL
                | SAVE_CONTROL
                | SAVE_BANK_ID
                | SOUND
                | CONTROLLER_TYPE
        )
    }
}

pub mod save_flags {
    pub const AUTO_SAVE: u8 = 4;
    pub const WRITE: u8 = 0;
}

pub mod interrupt_flags {
    pub const IRQ_INPUT: u8 = 0;
    pub const IRQ_LINE_DRAW: u8 = 1;
    pub const IRQ_SCREEN_DRAW: u8 = 2;
    pub const IRQ_TIMER: u8 = 3;
    pub const IRQ_CONTROLLER: u8 = 4;
    pub const IRQ_DATETIME: u8 = 5;
}

#[cfg(test)]
mod test {
    use crate::mem::sizes::{RESERVED, TOTAL};
    use crate::mem::{address, sizes};

    #[test]
    fn test_values() {
        assert_eq!(TOTAL, 65536);
        //the system needs at least 6 bytes available for internal use
        //currently this is just VM::execute_op()
        assert!(RESERVED > 6);
    }

    #[test]
    fn test_sizes_address() {
        assert_eq!(address::CODE, 0);
        assert_eq!(address::CODE_BANK, address::CODE + sizes::CODE_BANK);
        assert_eq!(address::RAM, address::CODE_BANK + sizes::CODE_BANK);
        assert_eq!(address::RAM_BANK, address::RAM + sizes::RAM_BANK);
        assert_eq!(address::INPUT, address::RAM_BANK + sizes::RAM_BANK);
        assert_eq!(address::SOUND, address::INPUT + sizes::INPUT);
        assert_eq!(address::SAVE_BANK_ID, address::SOUND + sizes::SOUND);
        assert_eq!(
            address::SAVE_BANK,
            address::SAVE_BANK_ID + sizes::SAVE_BANK_ID
        );
        assert_eq!(address::ATLAS1, address::SAVE_BANK + sizes::SAVE_BANK);
        assert_eq!(address::ATLAS2, address::ATLAS1 + sizes::ATLAS);
        assert_eq!(address::ATLAS3, address::ATLAS2 + sizes::ATLAS);
        assert_eq!(address::ATLAS4, address::ATLAS3 + sizes::ATLAS);
        assert_eq!(address::PALETTES, address::ATLAS4 + sizes::ATLAS);
        assert_eq!(
            address::SPRITE_TABLE,
            address::PALETTES + sizes::PALETTES_TOTAL
        );
        assert_eq!(
            address::LAYER_HEADERS,
            address::SPRITE_TABLE + sizes::SPRITE_TABLE
        );
        assert_eq!(
            address::LAYERS,
            address::LAYER_HEADERS + sizes::LAYERS_HEADER
        );
        assert_eq!(
            address::CODE_BANK_ID,
            address::LAYERS + sizes::LAYERS_CONTENT
        );
        assert_eq!(
            address::RAM_BANK_ID,
            address::CODE_BANK_ID + sizes::CODE_BANK_ID
        );
        assert_eq!(
            address::ATLAS1_BANK_ID,
            address::RAM_BANK_ID + sizes::RAM_BANK_ID
        );
        assert_eq!(
            address::ATLAS2_BANK_ID,
            address::ATLAS1_BANK_ID + sizes::ATLAS_BANK_ID
        );
        assert_eq!(
            address::ATLAS3_BANK_ID,
            address::ATLAS2_BANK_ID + sizes::ATLAS_BANK_ID
        );
        assert_eq!(
            address::ATLAS4_BANK_ID,
            address::ATLAS3_BANK_ID + sizes::ATLAS_BANK_ID
        );
        assert_eq!(address::SP, address::ATLAS4_BANK_ID + sizes::ATLAS_BANK_ID);
        assert_eq!(address::FP, address::SP + sizes::SP);
        assert_eq!(address::TIMER_CONTROL, address::FP + sizes::FP);
        assert_eq!(
            address::TIMER_VALUE1,
            address::TIMER_CONTROL + sizes::TIMER_CONTROL
        );
        assert_eq!(
            address::TIMER_VALUE2,
            address::TIMER_VALUE1 + sizes::TIMER_VALUE
        );
        assert_eq!(
            address::TIMER_VALUE3,
            address::TIMER_VALUE2 + sizes::TIMER_VALUE
        );
        assert_eq!(
            address::TIMER_VALUE4,
            address::TIMER_VALUE3 + sizes::TIMER_VALUE
        );
        assert_eq!(
            address::IRQ_RET_ADDR,
            address::TIMER_VALUE4 + sizes::TIMER_VALUE
        );
        assert_eq!(
            address::IRQ_REG_DUMP,
            address::IRQ_RET_ADDR + sizes::IRQ_RET_ADDR
        );
        assert_eq!(address::VLINE, address::IRQ_REG_DUMP + sizes::IRQ_REG_DUMP);
        assert_eq!(address::CONTROLLER_TYPE, address::VLINE + sizes::VLINE);
        assert_eq!(
            address::CONTROLLER_GRAPHICS,
            address::CONTROLLER_TYPE + sizes::CONTROLLER_TYPE
        );
        assert_eq!(
            address::CONTROLLER_PALETTE,
            address::CONTROLLER_GRAPHICS + sizes::CONTROLLER_GRAPHICS
        );
        assert_eq!(
            address::CONTROLLER_TABLE,
            address::CONTROLLER_PALETTE + sizes::CONTROLLER_PALETTE
        );
        assert_eq!(
            address::IRQ_CONTROL,
            address::CONTROLLER_TABLE + sizes::CONTROLLER_TABLE
        );
        assert_eq!(
            address::SAVE_CONTROL,
            address::IRQ_CONTROL + sizes::IRQ_CONTROL
        );
        assert_eq!(
            address::DATETIME,
            address::SAVE_CONTROL + sizes::SAVE_CONTROL
        );
        assert_eq!(address::RAND, address::DATETIME + sizes::DATETIME);
        assert_eq!(address::WAVE_TABLE, address::RAND + sizes::RAND);
        assert_eq!(address::RESERVED, address::WAVE_TABLE + sizes::WAVE_TABLE);
        assert_eq!(address::STACK, address::RESERVED + sizes::RESERVED);
        assert_eq!(65536, address::STACK as usize + sizes::STACK as usize);
        assert_eq!(address::MAX, 0xFFFF);
    }
}
