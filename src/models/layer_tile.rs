use crate::graphics::layer_tile::byte2;
use crate::models::{Byteable, LayerTile};

impl Byteable for LayerTile {
    const SIZE: usize = 2;

    fn from_bytes(bytes: &[u8]) -> LayerTile {
        LayerTile {
            id: bytes[0] as usize,
            flip_v: bytes[1] & byte2::MASK_FLIP_V == byte2::MASK_FLIP_V,
            flip_h: bytes[1] & byte2::MASK_FLIP_H == byte2::MASK_FLIP_H,
            palette: ((bytes[1] & byte2::MASK_PALETTE) >> byte2::OFFSET_PALETTE) as usize,
            half_alpha: bytes[1] & byte2::MASK_HALF_ALPHA == byte2::MASK_HALF_ALPHA,
            rotated: bytes[1] & byte2::MASK_ROTATED == byte2::MASK_ROTATED
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut byte2 = 0;
        if self.flip_h {
            byte2 |= byte2::MASK_FLIP_H;
        }
        if self.flip_v {
            byte2 |= byte2::MASK_FLIP_V;
        }
        if self.half_alpha {
            byte2 |= byte2::MASK_HALF_ALPHA;
        }
        if self.rotated {
            byte2 |= byte2::MASK_ROTATED;
        }
        byte2 |= (self.palette as u8) << byte2::OFFSET_PALETTE;
        vec![
            self.id as u8,
            byte2
        ]
    }
}