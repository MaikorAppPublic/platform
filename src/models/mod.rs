pub mod sprite;
pub mod layer_tile;
pub mod layer_header;

use std::io;
use std::io::ErrorKind;
use crate::constants::{TILES_PER_LAYER_COLUMN, TILES_PER_LAYER_ROW};

#[derive(Debug, Clone)]
pub struct Sprite {
    pub x: usize,
    pub y: usize,
    pub id: usize,
    pub flip_v: bool,
    pub flip_h: bool,
    pub palette: usize,
    pub large: bool,
    pub order: usize,
    pub half_alpha: bool,
    pub rotated: bool,
    pub atlas: usize,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub header: LayerHeader,
    pub content: [LayerTile; TILES_PER_LAYER_ROW  * TILES_PER_LAYER_COLUMN]
}

#[derive(Debug, Clone)]
pub struct LayerHeader {
    pub x: isize,
    pub y: isize,
    pub enabled: bool,
    pub atlas: usize,
    pub order: usize,
}

#[derive(Debug, Clone)]
pub struct LayerTile {
    pub id: usize,
    pub flip_v: bool,
    pub flip_h: bool,
    pub palette: usize,
    pub half_alpha: bool,
    pub rotated: bool
}

pub trait Byteable {
    const SIZE: usize;

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, io::Error> where Self: Sized {
        if bytes.len() < Self::SIZE {
            return Err(io::Error::from(ErrorKind::UnexpectedEof))
        } else {
            Ok(Self::from_bytes(bytes))
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self;

    fn to_bytes(&self) -> Vec<u8>;
}