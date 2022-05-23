use crate::graphics::sprite::{byte4, byte5};
use crate::models::{Byteable, Sprite};

impl Byteable for Sprite {
    const SIZE: usize = 5;

    fn from_bytes(bytes: &[u8]) -> Sprite {
        Sprite {
            x: bytes[0] as usize,
            y: bytes[1] as usize,
            id: bytes[2] as usize,
            flip_v: bytes[3] & byte4::MASK_FLIP_V == byte4::MASK_FLIP_V,
            flip_h: bytes[3] & byte4::MASK_FLIP_H == byte4::MASK_FLIP_H,
            palette: ((bytes[4] & byte5::MASK_PALETTE) >> 3) as usize,
            large: bytes[4] & byte5::MASK_LARGE == byte5::MASK_LARGE,
            order: ((bytes[4] & byte5::MASK_ORDER) >> 3) as usize,
            half_alpha: bytes[3] & byte4::MASK_HALF_ALPHA == byte4::MASK_HALF_ALPHA,
            rotated: bytes[3] & byte4::MASK_ROTATED == byte4::MASK_ROTATED,
            atlas: ((bytes[4] & byte5::MASK_ATLAS) >> 3) as usize,
            enabled: bytes[3] & byte4::MASK_ENABLED == byte4::MASK_ENABLED
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut byte4 = 0;
        let mut byte5 = 0;
        if self.flip_h {
            byte4 |= byte4::MASK_FLIP_H;
        }
        if self.flip_v {
            byte4 |= byte4::MASK_FLIP_V;
        }
        if self.half_alpha {
            byte4 |= byte4::MASK_HALF_ALPHA;
        }
        if self.enabled {
            byte4 |= byte4::MASK_ENABLED;
        }
        if self.rotated {
            byte4 |= byte4::MASK_ROTATED;
        }
        if self.large {
            byte5 |= byte5::MASK_LARGE;
        }
        byte5 |= (self.atlas >> byte5::OFFSET_ATLAS) as u8;
        byte5 |= (self.order >> byte5::OFFSET_ORDER) as u8;
        byte5 |= self.palette as u8;
        vec![
            self.x as u8,
            self.y as u8,
            self.id as u8,
            byte4,
            byte5
        ]
    }
}