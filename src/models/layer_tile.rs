use crate::graphics::layer_tile::byte2;
use crate::models::{Byteable, LayerTile};

impl LayerTile {
    pub fn new(
        id: usize,
        flip_v: bool,
        flip_h: bool,
        palette: usize,
        half_alpha: bool,
        rotated: bool,
    ) -> Self {
        Self {
            id,
            flip_v,
            flip_h,
            palette,
            half_alpha,
            rotated,
        }
    }
}

impl Byteable for LayerTile {
    const SIZE: usize = 2;

    fn from_bytes(bytes: &[u8]) -> LayerTile {
        LayerTile {
            id: bytes[0] as usize,
            flip_v: bytes[1] & byte2::MASK_FLIP_V == byte2::MASK_FLIP_V,
            flip_h: bytes[1] & byte2::MASK_FLIP_H == byte2::MASK_FLIP_H,
            palette: ((bytes[1] & byte2::MASK_PALETTE) >> byte2::OFFSET_PALETTE) as usize,
            half_alpha: bytes[1] & byte2::MASK_HALF_ALPHA == byte2::MASK_HALF_ALPHA,
            rotated: bytes[1] & byte2::MASK_ROTATED == byte2::MASK_ROTATED,
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
        vec![self.id as u8, byte2]
    }
}

#[cfg(test)]
mod test {
    use crate::graphics::layer_tile::byte2;
    use crate::models::{Byteable, LayerTile};

    #[test]
    fn write_test() {
        let tile = LayerTile::new(0, false, false, 0, false, false);
        assert_eq!(tile.to_bytes(), vec![0, 0]);

        let tile = LayerTile::new(255, true, true, 3, true, true);
        let bytes = tile.to_bytes();
        assert_eq!(bytes, vec![255, 252]);
        assert_eq!(bytes[1] & byte2::MASK_FLIP_V, byte2::MASK_FLIP_V);
        assert_eq!(bytes[1] & byte2::MASK_FLIP_H, byte2::MASK_FLIP_H);
        assert_eq!(bytes[1] & byte2::MASK_HALF_ALPHA, byte2::MASK_HALF_ALPHA);
        assert_eq!(bytes[1] & byte2::MASK_ROTATED, byte2::MASK_ROTATED);
        assert_eq!((bytes[1] & byte2::MASK_PALETTE) >> byte2::OFFSET_PALETTE, 3);

        let tile = LayerTile::new(58, true, false, 1, true, false);
        let bytes = tile.to_bytes();
        assert_eq!(bytes, vec![58, 152]);
        assert_eq!(bytes[1] & byte2::MASK_FLIP_V, byte2::MASK_FLIP_V);
        assert_eq!(bytes[1] & byte2::MASK_FLIP_H, 0);
        assert_eq!(bytes[1] & byte2::MASK_HALF_ALPHA, byte2::MASK_HALF_ALPHA);
        assert_eq!(bytes[1] & byte2::MASK_ROTATED, 0);
        assert_eq!((bytes[1] & byte2::MASK_PALETTE) >> byte2::OFFSET_PALETTE, 1);
    }

    #[test]
    fn read_test() {
        let tile = LayerTile::from_bytes(&[0, 0]);
        assert_eq!(tile, LayerTile::default());
        let tile = LayerTile::from_bytes(&[255, 252]);
        assert_eq!(tile, LayerTile::new(255, true, true, 3, true, true));
        let tile = LayerTile::from_bytes(&[91, byte2::MASK_FLIP_V | byte2::MASK_HALF_ALPHA]);
        assert_eq!(tile, LayerTile::new(91, true, false, 0, true, false));
    }
}
