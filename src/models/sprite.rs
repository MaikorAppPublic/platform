use crate::graphics::sprite::{byte4, byte5};
use crate::models::{Byteable, Sprite};

impl Sprite {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: usize,
        y: usize,
        id: usize,
        flip_v: bool,
        flip_h: bool,
        palette: usize,
        large: bool,
        order: usize,
        half_alpha: bool,
        rotated: bool,
        atlas: usize,
        enabled: bool,
    ) -> Self {
        Self {
            x,
            y,
            id,
            flip_v,
            flip_h,
            palette,
            large,
            order,
            half_alpha,
            rotated,
            atlas,
            enabled,
        }
    }
}

impl Byteable for Sprite {
    const SIZE: usize = 5;

    fn from_bytes(bytes: &[u8]) -> Sprite {
        Sprite {
            x: bytes[0] as usize,
            y: bytes[1] as usize,
            id: bytes[2] as usize,
            flip_v: bytes[3] & byte4::MASK_FLIP_V == byte4::MASK_FLIP_V,
            flip_h: bytes[3] & byte4::MASK_FLIP_H == byte4::MASK_FLIP_H,
            palette: (bytes[4] & byte5::MASK_PALETTE) as usize,
            large: bytes[4] & byte5::MASK_LARGE == byte5::MASK_LARGE,
            order: ((bytes[4] & byte5::MASK_ORDER) >> byte5::OFFSET_ORDER) as usize,
            half_alpha: bytes[3] & byte4::MASK_HALF_ALPHA == byte4::MASK_HALF_ALPHA,
            rotated: bytes[3] & byte4::MASK_ROTATED == byte4::MASK_ROTATED,
            atlas: ((bytes[4] & byte5::MASK_ATLAS) >> byte5::OFFSET_ATLAS) as usize,
            enabled: bytes[3] & byte4::MASK_ENABLED == byte4::MASK_ENABLED,
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
        byte5 |= (self.atlas << byte5::OFFSET_ATLAS) as u8;
        byte5 |= (self.order << byte5::OFFSET_ORDER) as u8;
        byte5 |= self.palette as u8;
        vec![self.x as u8, self.y as u8, self.id as u8, byte4, byte5]
    }
}

#[cfg(test)]
mod test {
    use crate::graphics::sprite::{byte4, byte5};
    use crate::models::{Byteable, Sprite};

    #[test]
    fn write_test() {
        let sprite = Sprite::default();
        assert_eq!(sprite.to_bytes(), [0, 0, 0, 0, 0]);
        let sprite = Sprite::new(255, 255, 255, true, true, 3, true, 3, true, true, 3, true);
        assert_eq!(sprite.to_bytes(), [255, 255, 255, 241, 251]);
        let sprite = Sprite::new(51, 120, 34, true, false, 1, true, 2, true, false, 0, true);
        let bytes = sprite.to_bytes();
        assert_eq!(bytes, [51, 120, 34, 161, 193]);
        assert_eq!(bytes[3] & byte4::MASK_ENABLED, byte4::MASK_ENABLED);
        assert_eq!(bytes[3] & byte4::MASK_FLIP_V, byte4::MASK_FLIP_V);
        assert_eq!(bytes[3] & byte4::MASK_FLIP_H, 0);
        assert_eq!(bytes[3] & byte4::MASK_ROTATED, 0);
        assert_eq!(bytes[3] & byte4::MASK_HALF_ALPHA, byte4::MASK_HALF_ALPHA);
        assert_eq!(bytes[4] & byte5::MASK_LARGE, byte5::MASK_LARGE);
        assert_eq!((bytes[4] & byte5::MASK_ORDER) >> byte5::OFFSET_ORDER, 2);
        assert_eq!(bytes[4] & byte5::MASK_PALETTE, 1);
        assert_eq!((bytes[4] & byte5::MASK_ATLAS) >> byte5::OFFSET_ATLAS, 0);
    }

    #[test]
    fn read_test() {
        let sprite = Sprite::from_bytes(&[0, 0, 0, 0, 0]);
        assert_eq!(sprite, Sprite::default());
        let sprite = Sprite::from_bytes(&[255, 255, 255, 255, 255]);
        assert_eq!(
            sprite,
            Sprite::new(255, 255, 255, true, true, 3, true, 3, true, true, 3, true)
        );
        let sprite = Sprite::from_bytes(&[
            126,
            69,
            99,
            byte4::MASK_ENABLED | byte4::MASK_FLIP_V | byte4::MASK_HALF_ALPHA,
            byte5::MASK_LARGE | 1,
        ]);
        assert_eq!(
            sprite,
            Sprite::new(126, 69, 99, true, false, 1, true, 0, true, false, 0, true)
        );
    }
}
