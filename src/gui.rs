use bracket_color::prelude::{ColorPair, RgbLerp, RGB, RGBA};
use bracket_geometry::prelude::{Point, Rect};
use bracket_lib::color;
use bracket_terminal::prelude::{
    render_draw_buffer, to_cp437, BResult, BTerm, BTermBuilder, DrawBatch,
};
use specs::prelude::*;
use std::cmp;

use super::camera::{get_camera_bounds_in_world, render_camera};
use super::components::{CombatStats, InInventory, Name, Player};
use super::map::Map;
use super::message_log::MessageLog;
use super::{InventoryMenuState, GAME_TITLE};

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

pub const FIRST_MESSAGE_COLOR: RGB = RGB {
    r: 156.0 / 255.0,
    g: 189.0 / 255.0,
    b: 181.0 / 255.0,
};
pub const LAST_MESSAGE_COLOR: RGB = RGB {
    r: 74.0 / 255.0,
    g: 115.0 / 255.0,
    b: 148.0 / 255.0,
};
pub const HEALTH_CRITICAL_COLOR: RGB = RGB {
    r: 231.0 / 255.0,
    g: 99.0 / 255.0,
    b: 82.0 / 255.0,
};
pub const HEALTH_OKAY_COLOR: RGB = RGB {
    r: 231.0 / 255.0,
    g: 188.0 / 255.0,
    b: 82.0 / 255.0,
};
pub const HEALTH_GOOD_COLOR: RGB = RGB {
    r: 107.0 / 255.0,
    g: 148.0 / 255.0,
    b: 82.0 / 255.0,
};
pub const TRANSPARENT: RGBA = RGBA {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub const MENU_TEXT_COLOR: RGB = FIRST_MESSAGE_COLOR;
pub const MENU_LINE_COLOR: RGB = LAST_MESSAGE_COLOR;
pub const MENU_HIGHLIGHT_COLOR: RGB = HEALTH_OKAY_COLOR;
pub const MENU_BG_COLOR: RGBA = RGBA {
    r: 27.0 / 255.0,
    g: 24.0 / 255.0,
    b: 25.0 / 255.0,
    a: 1.0,
};

pub enum Consoles {
    TilesTerrain,
    TilesEntitiesItems,
    TilesEntitiesCharacters,
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

pub fn draw_box_with_filled_bg(batch: &mut DrawBatch, bounds: Rect, colorpair: ColorPair) {
    batch.fill_region(
        Rect::with_exact(bounds.x1, bounds.y1, bounds.x2 + 1, bounds.y2 + 1),
        ColorPair::new(colorpair.bg, colorpair.bg),
        to_cp437('█'),
    );
    batch.draw_box(bounds, colorpair);
}

pub fn print_color_with_filled_bg<S: ToString>(
    batch: &mut DrawBatch,
    pos: Point,
    text: S,
    colorpair: ColorPair,
    left_padding: i32,
    right_padding: i32,
) {
    let as_string = text.to_string();
    // FIXME: this will be wrong for multi-byte characters
    let width = as_string.len() as i32;
    let fill_bounds = Rect::with_size(pos.x - left_padding, pos.y, width + right_padding + 1, 1);
    // FIXME: assert this is still in the screen space
    batch.fill_region(
        fill_bounds,
        ColorPair::new(colorpair.bg, colorpair.bg),
        to_cp437('█'),
    );
    batch.print_color(pos, text, colorpair);
}

#[derive(Debug)]
pub struct MainView {
    pub camera_view_2x: Rect,
    pub message_log_view: Rect,
    pub stats_view: Rect,
    pub window_in_tiles: Rect,
    pub window_in_text: Rect,
    pub mouse_pt_in_tiles: Point,
    pub mouse_pt_in_text: Point,
}

impl MainView {
    pub fn from_context(ctx: &mut BTerm) -> MainView {
        ctx.set_active_console(Consoles::TilesTerrain as usize);
        let (width_in_tiles, height_in_tiles) = ctx.get_char_size();
        let mouse_pt_in_tiles = ctx.mouse_point();
        ctx.set_active_console(Consoles::Text as usize);
        let (width_in_text, height_in_text) = ctx.get_char_size();
        let message_log_width = width_in_text / 2;
        let stats_width = width_in_text - message_log_width;
        let mouse_pt_in_text = ctx.mouse_point();

        MainView {
            camera_view_2x: Rect::with_size(
                0,
                0,
                width_in_tiles,
                if height_in_tiles > TEXT_BOX_HEIGHT_IN_TILES {
                    height_in_tiles - TEXT_BOX_HEIGHT_IN_TILES
                } else {
                    height_in_tiles
                },
            ),
            message_log_view: Rect::with_size(
                0,
                height_in_text - TEXT_BOX_HEIGHT,
                message_log_width,
                TEXT_BOX_HEIGHT,
            ),
            stats_view: Rect::with_size(
                message_log_width,
                height_in_text - TEXT_BOX_HEIGHT,
                stats_width,
                TEXT_BOX_HEIGHT,
            ),
            window_in_tiles: Rect::with_size(0, 0, width_in_tiles, height_in_tiles),
            window_in_text: Rect::with_size(0, 0, width_in_text, height_in_text),
            mouse_pt_in_tiles: mouse_pt_in_tiles,
            mouse_pt_in_text: mouse_pt_in_text,
        }
    }
}

pub fn render_main_view(ecs: &World, ctx: &mut BTerm) {
    let main_view = MainView::from_context(ctx);
    let maybe_camera_in_world = get_camera_bounds_in_world(ecs, main_view.camera_view_2x);
    if maybe_camera_in_world.is_none() {
        // No player yet?
        return;
    }
    let camera_in_world = maybe_camera_in_world.unwrap();

    render_camera(
        ecs,
        main_view.camera_view_2x,
        camera_in_world,
        main_view.window_in_tiles,
    );
    let mut batch = DrawBatch::new();
    batch.target(Consoles::Text as usize);
    batch.cls();
    render_messages(ecs, &mut batch, main_view.message_log_view);
    render_stats(ecs, &mut batch, main_view.stats_view);
    render_tooltips(
        &mut batch,
        ecs,
        main_view.camera_view_2x,
        camera_in_world,
        main_view.mouse_pt_in_tiles,
        main_view.mouse_pt_in_text,
    );

    render_draw_buffer(ctx).expect("Couldn't render camera");
}

pub fn render_messages(ecs: &World, batch: &mut DrawBatch, bounds: Rect) {
    let message_log = ecs.fetch::<MessageLog>();
    let tail = message_log
        .entries
        .iter()
        .rev()
        .take(bounds.height() as usize);
    let mut color_lerp = RgbLerp::new(
        FIRST_MESSAGE_COLOR,
        LAST_MESSAGE_COLOR,
        bounds.height() as usize,
    );

    batch.target(Consoles::Text as usize);
    for (line_no, entry) in tail.enumerate() {
        batch.print_color(
            Point {
                x: bounds.x1,
                y: bounds.y1 + (line_no as i32),
            },
            entry,
            ColorPair {
                fg: color_lerp.next().unwrap_or(LAST_MESSAGE_COLOR).to_rgba(1.0),
                bg: TRANSPARENT,
            },
        );
    }
    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't render message log");
}

pub fn render_stats(ecs: &World, batch: &mut DrawBatch, bounds: Rect) {
    let stats_store = ecs.read_storage::<CombatStats>();
    let player_store = ecs.read_storage::<Player>();

    batch.target(Consoles::Text as usize);

    for (_player, stats) in (&player_store, &stats_store).join() {
        let health_text = format!("HP: {} / {}", stats.hp, stats.max_hp);
        let health_portion: f32 = stats.hp as f32 / stats.max_hp as f32;
        let health_color = ColorPair {
            fg: if health_portion < 0.2 {
                HEALTH_CRITICAL_COLOR.to_rgba(1.0)
            } else if health_portion < 0.5 {
                HEALTH_OKAY_COLOR.to_rgba(1.0)
            } else {
                HEALTH_GOOD_COLOR.to_rgba(1.0)
            },
            bg: TRANSPARENT,
        };
        batch.print_color(
            Point {
                x: bounds.x1,
                y: bounds.y1 + 1,
            },
            &health_text,
            health_color,
        );
        let health_bar_offset = health_text.len() as i32 + 4;
        let health_bar_width = bounds.width() - health_bar_offset - 4;
        if health_bar_width > 0 {
            batch.bar_horizontal(
                Point {
                    x: bounds.x1 + health_bar_offset,
                    y: bounds.y1 + 1,
                },
                health_bar_width,
                stats.hp,
                stats.max_hp,
                health_color,
            );
        }
    }

    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't render player stats");
}

pub fn render_tooltips(
    batch: &mut DrawBatch,
    ecs: &World,
    camera_view: Rect,
    camera_in_world: Rect,
    mouse_pt_in_tiles: Point,
    mouse_pt_in_text: Point,
) {
    let names = ecs.read_storage::<Name>();
    let map = ecs.fetch::<Map>();

    if !camera_view.point_in_rect(mouse_pt_in_tiles) {
        // Pointing out of camera
        return;
    }

    let mouse_pt_in_world: Point = mouse_pt_in_tiles
        + Point {
            x: camera_in_world.x1,
            y: camera_in_world.y1,
        };
    if !map.bounds().point_in_rect(mouse_pt_in_world) {
        // Pointing out of map
        return;
    }

    let mouse_pt_as_index = map.to_index(mouse_pt_in_world);
    if !map.visible_tiles[mouse_pt_as_index] {
        // Point out of player vision
        return;
    }

    let entities_at_tile = map.tile_content[mouse_pt_as_index].clone();
    if entities_at_tile.is_empty() {
        // Not pointing at an entity
        return;
    }

    let mut tooltips: Vec<String> = Vec::new();

    batch.target(Consoles::Text as usize);

    for entity in entities_at_tile.iter() {
        let maybe_name = names.get(*entity);
        if let Some(name) = maybe_name {
            tooltips.push(name.name.to_string());
        }
    }
    for tooltip in tooltips.iter() {
        batch.print_color(
            mouse_pt_in_text + Point { x: 4, y: 0 },
            tooltip,
            ColorPair {
                fg: FIRST_MESSAGE_COLOR.to_rgba(1.0),
                bg: RGBA {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            },
        );
    }

    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't render text");
}

#[derive(Debug)]
pub struct MenuInventoryView {
    pub camera_view_2x: Rect,
    pub menu_view: Rect,
    pub window_in_tiles: Rect,
    pub window_in_text: Rect,
    pub mouse_pt_in_tiles: Point,
    pub mouse_pt_in_text: Point,
}

impl MenuInventoryView {
    pub fn from_context(ctx: &mut BTerm) -> MenuInventoryView {
        ctx.set_active_console(Consoles::TilesTerrain as usize);
        let (width_in_tiles, height_in_tiles) = ctx.get_char_size();
        let mouse_pt_in_tiles = ctx.mouse_point();
        ctx.set_active_console(Consoles::Text as usize);
        let (width_in_text, height_in_text) = ctx.get_char_size();
        let mouse_pt_in_text = ctx.mouse_point();

        MenuInventoryView {
            camera_view_2x: Rect::with_size(
                0,
                0,
                width_in_tiles,
                if height_in_tiles > TEXT_BOX_HEIGHT_IN_TILES {
                    height_in_tiles - TEXT_BOX_HEIGHT_IN_TILES
                } else {
                    height_in_tiles
                },
            ),
            menu_view: Rect::with_size(0, 0, width_in_text, height_in_text - TEXT_BOX_HEIGHT),
            window_in_tiles: Rect::with_size(0, 0, width_in_tiles, height_in_tiles),
            window_in_text: Rect::with_size(0, 0, width_in_text, height_in_text),
            mouse_pt_in_tiles: mouse_pt_in_tiles,
            mouse_pt_in_text: mouse_pt_in_text,
        }
    }
}

pub fn render_inventory_menu(ecs: &World, ctx: &mut BTerm, _menu_state: InventoryMenuState) {
    let view = MenuInventoryView::from_context(ctx);
    let player = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let held_items = ecs.read_storage::<InInventory>();

    let mut batch = DrawBatch::new();
    batch.target(Consoles::Text as usize);

    let text_colorpair = ColorPair::new(MENU_TEXT_COLOR.to_rgba(1.0), MENU_BG_COLOR);
    let highlight_colorpair = ColorPair::new(MENU_HIGHLIGHT_COLOR.to_rgba(1.0), MENU_BG_COLOR);
    let line_colorpair = ColorPair::new(MENU_LINE_COLOR.to_rgba(1.0), MENU_BG_COLOR);

    let inventory = (&held_items, &names)
        .join()
        .filter(|item| item.0.owner == *player);
    let count = inventory.clone().count() as i32;

    let menu_bounds_abstract = Rect::with_exact(0, 0, 40, cmp::max(count, 1) + 3);

    let menu_x = (view.menu_view.width() - menu_bounds_abstract.width()) / 2;
    let menu_y = (view.menu_view.height() - menu_bounds_abstract.height()) / 2;
    let menu_bounds = Rect::with_exact(
        menu_x,
        menu_y,
        menu_x + menu_bounds_abstract.width(),
        menu_y + menu_bounds_abstract.height(),
    );
    let listing_bounds = Rect::with_exact(
        menu_bounds.x1 + 2,
        menu_bounds.y1 + 2,
        menu_bounds.x2 - 2,
        menu_bounds.y2 - 2,
    );
    // FIXME: assert all these rects are in screenspace

    draw_box_with_filled_bg(&mut batch, menu_bounds, line_colorpair);
    print_color_with_filled_bg(
        &mut batch,
        Point::new(menu_bounds.x1 + 3, menu_bounds.y1),
        "Inventory",
        highlight_colorpair,
        1,
        1,
    );
    print_color_with_filled_bg(
        &mut batch,
        Point::new(menu_bounds.x1 + 3, menu_bounds.y2),
        "<escape> to cancel",
        highlight_colorpair,
        1,
        1,
    );

    if count == 0 {
        batch.print_color(
            Point::new(listing_bounds.x1, listing_bounds.y1),
            "No items",
            text_colorpair,
        );
    } else {
        let mut y = listing_bounds.y1;
        for (_item, item_name) in inventory {
            batch.print_color(
                Point::new(listing_bounds.x1, y),
                &item_name.name.to_string(),
                text_colorpair,
            );
            y += 1;
        }
    }

    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't draw inventory listing");

    render_draw_buffer(ctx).expect("Couldn't render inventory listing");
}
