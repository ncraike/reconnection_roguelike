use bracket_color::prelude::RGB;

use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;
use std::cmp;

use super::super::components::{InInventory, Name};
use super::super::InventoryMenuState;

use super::colors;
use super::common::{Consoles, TEXT_BOX_HEIGHT, TEXT_BOX_HEIGHT_IN_TILES};
use super::utils::{draw_box_with_filled_bg, print_color_with_filled_bg};

const MENU_TEXT_COLOR: RGB = colors::BLUE_LIGHT;
const MENU_LINE_COLOR: RGB = colors::BLUE_MID_DARK;
const MENU_HIGHLIGHT_COLOR: RGB = colors::BRIGHT_YELLOW;
const MENU_BG_COLOR: RGB = colors::BROWN_DARK;

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
