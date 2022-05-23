pub mod layer_header;
pub mod layer_tile;
pub mod sprite;

use crate::constants::TILES_PER_LAYER;
use std::io;
use std::io::ErrorKind;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
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
    pub content: [LayerTile; TILES_PER_LAYER],
}

impl Layer {
    pub fn new(header: LayerHeader, content: [LayerTile; TILES_PER_LAYER]) -> Self {
        Self { header, content }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct LayerHeader {
    pub x: isize,
    pub y: isize,
    pub enabled: bool,
    pub atlas: usize,
    pub order: usize,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct LayerTile {
    pub id: usize,
    pub flip_v: bool,
    pub flip_h: bool,
    pub palette: usize,
    pub half_alpha: bool,
    pub rotated: bool,
}

pub trait Byteable {
    const SIZE: usize;

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, io::Error>
    where
        Self: Sized,
    {
        if bytes.len() < Self::SIZE {
            Err(io::Error::from(ErrorKind::UnexpectedEof))
        } else {
            Ok(Self::from_bytes(bytes))
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self;

    fn to_bytes(&self) -> Vec<u8>;
}
