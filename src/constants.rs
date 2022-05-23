/// Width of screen in pixels
pub const SCREEN_WIDTH: usize = 240;
/// Height of screen in pixels
pub const SCREEN_HEIGHT: usize = 160;
/// Total pixel count for screen (width * height)
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
/// Max sprites on screen
pub const SPRITE_COUNT: usize = 255;
/// Number of background layers
pub const LAYER_COUNT: usize = 3;
/// Number of palettes
pub const PALETTE_COUNT: usize = 4;
/// Tiles per row on screen
pub const TILES_PER_ROW: usize = 30;
/// Tiles per column on screen
pub const TILES_PER_COLUMN: usize = 20;
/// Tiles per row on a background layer
pub const TILES_PER_LAYER_ROW: usize = 44;
/// Tiles per row on a background layer
pub const TILES_PER_LAYER_COLUMN: usize = 30;
/// Tile count per layer
pub const TILES_PER_LAYER: usize = TILES_PER_LAYER_ROW * TILES_PER_LAYER_COLUMN;
/// Tiles per row in an atlas
pub const TILES_PER_ATLAS_ROW: usize = 25;
/// Tiles per row in an atlas
pub const TILES_PER_ATLAS_COLUMN: usize = 20;
/// Width of tiles in pixels
pub const TILE_WIDTH: usize = 8;
/// Height of tiles in pixels
pub const TILE_HEIGHT: usize = 8;
/// Width of tiles in bytes in an atlas
pub const ATLAS_TILE_WIDTH: usize = 4;
/// Height of tiles in bytes in an atlas
pub const ATLAS_TILE_HEIGHT: usize = 8;
/// Number of save files
pub const SAVE_COUNT: usize = 16;
