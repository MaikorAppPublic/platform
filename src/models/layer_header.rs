use crate::graphics::layer_header::byte3;
use crate::models::{Byteable, LayerHeader};

impl Byteable for LayerHeader {
    const SIZE: usize = 3;

    fn from_bytes(bytes: &[u8]) -> LayerHeader {
        LayerHeader {
            x: bytes[0] as i8 as isize,
            y: bytes[1] as i8 as isize,
            enabled: bytes[2] & byte3::MASK_ENABLED == byte3::MASK_ENABLED,
            order: ((bytes[2] & byte3::MASK_ORDER) >> byte3::OFFSET_ORDER) as usize,
            atlas:((bytes[2] & byte3::MASK_ATLAS) >> byte3::OFFSET_ATLAS) as usize,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut byte3 = 0;
        if self.enabled {
            byte3 |= byte3::MASK_ENABLED;
        }
        byte3 |= (self.order as u8) << byte3::OFFSET_ORDER;
        byte3 |= (self.atlas as u8) << byte3::OFFSET_ATLAS;
        vec![
            self.x as i8 as u8,
            self.y as i8 as u8,
            byte3
        ]
    }
}