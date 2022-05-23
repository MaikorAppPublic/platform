use crate::graphics::layer_header::byte3;
use crate::models::{Byteable, LayerHeader};

impl LayerHeader {
    pub fn new(x: isize, y: isize, enabled: bool, atlas: usize, order: usize) -> Self {
        Self { x, y, enabled, atlas, order }
    }
}

impl Byteable for LayerHeader {
    const SIZE: usize = 3;

    fn from_bytes(bytes: &[u8]) -> LayerHeader {
        LayerHeader {
            x: bytes[0] as i8 as isize,
            y: bytes[1] as i8 as isize,
            enabled: bytes[2] & byte3::MASK_ENABLED == byte3::MASK_ENABLED,
            order: ((bytes[2] & byte3::MASK_ORDER) >> byte3::OFFSET_ORDER) as usize,
            atlas: ((bytes[2] & byte3::MASK_ATLAS) >> byte3::OFFSET_ATLAS) as usize,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut byte3 = 0;
        if self.enabled {
            byte3 |= byte3::MASK_ENABLED;
        }
        byte3 |= (self.order as u8) << byte3::OFFSET_ORDER;
        byte3 |= (self.atlas as u8) << byte3::OFFSET_ATLAS;
        vec![self.x as i8 as u8, self.y as i8 as u8, byte3]
    }
}

#[cfg(test)]
mod test {
    use crate::graphics::layer_header::byte3;
    use crate::models::{Byteable, LayerHeader};

    #[test]
    fn write_test() {
        let header = LayerHeader::new(0,0,false,0,0);
        assert_eq!(header.to_bytes(), [0,0,0]);
        let header = LayerHeader::new(125,12,true,1,2);
        let bytes = header.to_bytes();
        assert_eq!(bytes, [125,12,19]);
        assert_eq!(bytes[2] & byte3::MASK_ENABLED, byte3::MASK_ENABLED);
        assert_eq!((bytes[2] & byte3::MASK_ATLAS) >> byte3::OFFSET_ATLAS, 1);
        assert_eq!((bytes[2] & byte3::MASK_ORDER) >> byte3::OFFSET_ORDER, 2);
        let header = LayerHeader::new(-125,-77,false,2,3);
        let bytes = header.to_bytes();
        assert_eq!(bytes, [131,179,28]);
        assert_eq!(bytes[2] & byte3::MASK_ENABLED, 0);
        assert_eq!((bytes[2] & byte3::MASK_ATLAS) >> byte3::OFFSET_ATLAS, 2);
        assert_eq!((bytes[2] & byte3::MASK_ORDER) >> byte3::OFFSET_ORDER, 3);
    }

    #[test]
    fn read_test() {
        let tile = LayerHeader::from_bytes(&[0,0,0]);
        assert_eq!(tile, LayerHeader::default());
        let tile = LayerHeader::from_bytes(&[127,127,255]);
        assert_eq!(tile, LayerHeader::new(127, 127, true, 3, 3));
        let tile = LayerHeader::from_bytes(&[255,255,0]);
        assert_eq!(tile, LayerHeader::new(-1, -1, false, 0,0));
        let tile = LayerHeader::from_bytes(&[12,204, byte3::MASK_ENABLED]);
        assert_eq!(tile, LayerHeader::new(12,-52, true, 0,0));
    }
}