use bracket_color::prelude::{ColorPair, RgbLerp, RGB, RGBA};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BResult, BTerm, BTermBuilder, DrawBatch};
use specs::prelude::*;

use super::camera::render_camera;
use super::message_log::MessageLog;
use super::GAME_TITLE;

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

pub const MESSAGES_HEIGHT: u32 = 9;
pub const MESSAGES_HEIGHT_IN_TILES: u32 = MESSAGES_HEIGHT * TEXT_FONT_HEIGHT / TILE_2X_HEIGHT;

pub const TILE_1X_FONT: &str = "reconnection_16x24_tiles_at_1x.png";
pub const TILE_2X_FONT: &str = "reconnection_16x24_tiles_at_2x.png";
pub const TEXT_FONT: &str = "vga8x16.png";

pub const MESSAGE_LOG_FIRST_COLOR: RGB = RGB {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
pub const MESSAGE_LOG_LAST_COLOR: RGB = RGB {
    r: 0.4,
    g: 0.4,
    b: 0.4,
};
pub const TRANSPARENT: RGBA = RGBA {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub enum Consoles {
    TilesTerrain,
    TilesEntities,
    Text,
}

pub fn build_terminal() -> BResult<BTerm> {
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
        ecs,
        &mut batch,
        main_view.message_log_view,
        main_view.window_in_text,
    );

    render_draw_buffer(ctx).expect("Couldn't render camera");
}

pub fn render_messages(ecs: &World, batch: &mut DrawBatch, bounds: Rect, _window_bounds: Rect) {
    let message_log = ecs.fetch::<MessageLog>();
    let tail = message_log
        .entries
        .iter()
        .rev()
        .take(bounds.height() as usize);
    let mut color_lerp = RgbLerp::new(
        MESSAGE_LOG_FIRST_COLOR,
        MESSAGE_LOG_LAST_COLOR,
        bounds.height() as usize,
    );

    batch.target(Consoles::Text as usize);
    batch.cls();
    for (line_no, entry) in tail.enumerate() {
        batch.print_color(
            Point {
                x: bounds.x1,
                y: bounds.y1 + (line_no as i32),
            },
            entry,
            ColorPair {
                fg: color_lerp
                    .next()
                    .unwrap_or(MESSAGE_LOG_LAST_COLOR)
                    .to_rgba(1.0),
                bg: TRANSPARENT,
            },
        );
    }
    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't render message log");
}
