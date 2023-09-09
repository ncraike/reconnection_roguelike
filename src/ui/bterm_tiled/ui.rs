use bracket_terminal::prelude::{BResult, BTerm, BTermBuilder};

use super::super::main::GAME_TITLE;
use super::types::UI;

pub const DEFAULT_WINDOW_WIDTH_IN_TILES: u32 = 48;
pub const DEFAULT_WINDOW_HEIGHT_IN_TILES: u32 = 18;

pub const TILE_1X_WIDTH: u32 = 16;
pub const TILE_1X_HEIGHT: u32 = 24;
pub const TILE_2X_WIDTH: u32 = 32;
pub const TILE_2X_HEIGHT: u32 = 48;
pub const TEXT_FONT_WIDTH: u32 = 8;
pub const TEXT_FONT_HEIGHT: u32 = 16;

pub const DEFAULT_WINDOW_WIDTH_IN_TEXT: u32 =
    DEFAULT_WINDOW_WIDTH_IN_TILES * TILE_2X_WIDTH / TEXT_FONT_WIDTH;
pub const DEFAULT_WINDOW_HEIGHT_IN_TEXT: u32 =
    DEFAULT_WINDOW_HEIGHT_IN_TILES * TILE_2X_HEIGHT / TEXT_FONT_HEIGHT;

pub const TEXT_BOX_HEIGHT: u32 = 6;
pub const TEXT_BOX_HEIGHT_IN_TILES: u32 = TEXT_BOX_HEIGHT * TEXT_FONT_HEIGHT / TILE_2X_HEIGHT;

pub const TILE_1X_FONT: &str = "reconnection_16x24_tiles_at_1x.png";
pub const TILE_2X_FONT: &str = "reconnection_16x24_tiles_at_2x.png";
pub const TEXT_FONT: &str = "vga8x16.png";
pub enum Consoles {
    TilesTerrain,
    TilesEntitiesItems,
    TilesEntitiesCharacters,
    Text,
}

pub struct BTermTiledUI {}

impl UI for BTermTiledUI {
    type Context = BTerm;
    type ContextResult = BResult;

    fn build_context() -> BResult<BTerm> {
        BTermBuilder::new()
            .with_title(GAME_TITLE)
            .with_tile_dimensions(TILE_2X_WIDTH, TILE_2X_HEIGHT)
            .with_dimensions(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
            )
            .with_automatic_console_resize(true)
            .with_fitscreen(true)
            .with_font(TILE_2X_FONT, TILE_2X_WIDTH, TILE_2X_HEIGHT)
            .with_font(TEXT_FONT, TEXT_FONT_WIDTH, TEXT_FONT_HEIGHT)
            // Terrain
            .with_simple_console(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
                TILE_2X_FONT,
            )
            // Entities (items)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
                TILE_2X_FONT,
            )
            // Entities (player, NPCs, enemies)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
                TILE_2X_FONT,
            )
            // Text
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_WIDTH_IN_TEXT,
                DEFAULT_WINDOW_HEIGHT_IN_TEXT,
                TEXT_FONT,
            )
            .build()
    }
}
