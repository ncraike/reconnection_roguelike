use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BResult, BTerm, BTermBuilder, DrawBatch};
use specs::prelude::*;

use super::camera::render_camera;
use super::GAME_TITLE;

pub const DEFAULT_WINDOW_WIDTH_IN_TILES: u32 = 50;
pub const DEFAULT_WINDOW_HEIGHT_IN_TILES: u32 = 20;

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

pub const MESSAGES_HEIGHT: u32 = 9;
pub const MESSAGES_HEIGHT_IN_TILES: u32 = MESSAGES_HEIGHT * TEXT_FONT_HEIGHT / TILE_2X_HEIGHT;

pub const TILE_1X_FONT: &str = "reconnection_16x24_tiles_at_1x.png";
pub const TILE_2X_FONT: &str = "reconnection_16x24_tiles_at_2x.png";
pub const TEXT_FONT: &str = "vga8x16.png";

pub enum Consoles {
    TilesTerrain,
    TilesEntities,
    Text,
}

pub fn build_terminal() -> BResult<BTerm> {
    BTermBuilder::new()
        .with_title(GAME_TITLE)
        .with_automatic_console_resize(true)
        .with_fitscreen(true)
        .with_font(TILE_2X_FONT, TILE_2X_WIDTH, TILE_2X_HEIGHT)
        .with_font(TEXT_FONT, TEXT_FONT_WIDTH, TEXT_FONT_HEIGHT)
        .with_simple_console(
            DEFAULT_WINDOW_WIDTH_IN_TILES,
            DEFAULT_WINDOW_HEIGHT_IN_TILES,
            TILE_2X_FONT,
        )
        .with_sparse_console_no_bg(
            DEFAULT_WINDOW_WIDTH_IN_TILES,
            DEFAULT_WINDOW_HEIGHT_IN_TILES,
            TILE_2X_FONT,
        )
        .with_sparse_console_no_bg(
            DEFAULT_WINDOW_WIDTH_IN_TEXT,
            DEFAULT_WINDOW_HEIGHT_IN_TEXT,
            TEXT_FONT,
        )
        .build()
}

#[derive(Debug)]
pub struct MainView {
    pub camera_view_2x: Rect,
    pub message_log_view: Rect,
    pub window_in_tiles: Rect,
    pub window_in_text: Rect,
}

impl MainView {
    pub fn from_context(ctx: &mut BTerm) -> MainView {
        ctx.set_active_console(Consoles::TilesTerrain as usize);
        let (width_in_tiles, height_in_tiles) = ctx.get_char_size();
        ctx.set_active_console(Consoles::Text as usize);
        let (width_in_text, height_in_text) = ctx.get_char_size();

        MainView {
            camera_view_2x: Rect::with_size(
                0,
                0,
                width_in_tiles,
                if height_in_tiles > MESSAGES_HEIGHT_IN_TILES {
                    height_in_tiles - MESSAGES_HEIGHT_IN_TILES
                } else {
                    height_in_tiles
                },
            ),
            message_log_view: Rect::with_size(
                0,
                height_in_text - MESSAGES_HEIGHT,
                width_in_text,
                MESSAGES_HEIGHT,
            ),
            window_in_tiles: Rect::with_size(0, 0, width_in_tiles, height_in_tiles),
            window_in_text: Rect::with_size(0, 0, width_in_text, height_in_text),
        }
    }
}

pub fn render_main_view(ecs: &World, ctx: &mut BTerm) {
    let main_view = MainView::from_context(ctx);
    let mut batch = DrawBatch::new();
    render_camera(
        ecs,
        &mut batch,
        main_view.camera_view_2x,
        main_view.window_in_tiles,
    );
    render_messages(
        &mut batch,
        main_view.message_log_view,
        main_view.window_in_text,
    );

    render_draw_buffer(ctx).expect("Couldn't render camera");
}

pub fn render_messages(batch: &mut DrawBatch, bounds: Rect, _window_bounds: Rect) {
    let color_pair = ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.));
    batch.target(Consoles::Text as usize);
    batch.cls();
    for line_no in 1..(bounds.height() + 1) {
        batch.print_color(
            Point {
                x: bounds.x1,
                y: bounds.y1 + line_no - 1,
            },
            format!("Line {}", line_no),
            color_pair,
        );
    }
    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't render message log");
}
