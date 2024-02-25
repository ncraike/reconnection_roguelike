use bracket_color::prelude::RGB;

use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;
use std::cmp;
use units::Box2DI32;
use units::HeightI32;
use units::PosYI32;
use units::Position2DI32;

use crate::components::{InInventory, Name};

use crate::ui::colors;
use crate::ui::common::{Consoles, TEXT_BOX_HEIGHT};
use crate::ui::units::ScreenChars;
use crate::ui::utils::{draw_box_with_filled_bg, print_color_with_filled_bg};

const MENU_TEXT_COLOR: RGB = colors::BLUE_LIGHT;
const MENU_LINE_COLOR: RGB = colors::BLUE_MID_DARK;
const MENU_HIGHLIGHT_COLOR: RGB = colors::BRIGHT_YELLOW;
const MENU_BG_COLOR: RGB = colors::BROWN_DARK;

#[derive(Debug, Clone)]
pub struct MenuInventoryView {
    pub camera_view: Box2DI32<ScreenChars>,
    pub menu_view: Box2DI32<ScreenChars>,
    pub window: Box2DI32<ScreenChars>,
    pub mouse_position: Position2DI32<ScreenChars>,
}

impl MenuInventoryView {
    pub fn from_context(ctx: &mut BTerm) -> MenuInventoryView {
        ctx.set_active_console(Consoles::UIText as usize);
        let mouse_position = ScreenChars::new_position2d_from_point(ctx.mouse_point());
        let (window_width, window_height) = ctx.get_char_size();
        let window =
            ScreenChars::new_box2d_from_width_height(window_width as i32, window_height as i32);

        MenuInventoryView {
            camera_view: ScreenChars::new_box2d(window.p1, window.p2 - TEXT_BOX_HEIGHT),
            menu_view: ScreenChars::new_box2d(window.p1, window.p2 - TEXT_BOX_HEIGHT),
            window: window,
            mouse_position: mouse_position,
        }
    }
}

pub fn render_inventory_menu(ecs: &World, ctx: &mut BTerm) {
    let view = MenuInventoryView::from_context(ctx);
    let player = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let held_items = ecs.read_storage::<InInventory>();

    let mut batch = DrawBatch::new();
    batch.target(Consoles::UIText as usize);

    let text_colorpair = ColorPair::new(MENU_TEXT_COLOR.to_rgba(1.0), MENU_BG_COLOR);
    let highlight_colorpair = ColorPair::new(MENU_HIGHLIGHT_COLOR.to_rgba(1.0), MENU_BG_COLOR);
    let line_colorpair = ColorPair::new(MENU_LINE_COLOR.to_rgba(1.0), MENU_BG_COLOR);

    let inventory = (&held_items, &names)
        .join()
        .filter(|item| item.0.owner == *player);
    let item_count = inventory.clone().count() as i32;

    let menu_size = ScreenChars::new_size2d(40, cmp::max(item_count + 3, 4));
    let menu_bounds =
        ScreenChars::new_box2d_from_position_and_size(view.menu_view.p1 + menu_size / 2, menu_size);
    let listing_padding = ScreenChars::new_size2d(2, 2);
    let listing_bounds = ScreenChars::new_box2d(
        menu_bounds.p1 + listing_padding,
        menu_bounds.p2 - listing_padding,
    );
    // FIXME: assert all these boxes are in screenspace

    draw_box_with_filled_bg(&mut batch, menu_bounds, line_colorpair);
    print_color_with_filled_bg(
        &mut batch,
        menu_bounds.p1 + ScreenChars::new_width(3),
        "Inventory",
        highlight_colorpair,
        ScreenChars::new_width(1),
        ScreenChars::new_width(1),
    );
    print_color_with_filled_bg(
        &mut batch,
        (menu_bounds.p1 + ScreenChars::new_width(3)).with_y_of(menu_bounds.p2),
        "<escape> to cancel",
        highlight_colorpair,
        ScreenChars::new_width(1),
        ScreenChars::new_width(1),
    );

    if item_count == 0 {
        batch.print_color(
            listing_bounds.p1.to_bracket_geometry_point(),
            "No items",
            text_colorpair,
        );
    } else {
        // FIXME: there has to be a better way to enumerate inventory
        let mut y = listing_bounds.y1().to_primitive();
        for (_item, item_name) in inventory {
            batch.print_color(
                listing_bounds
                    .p1
                    .with_y(PosYI32(ScreenChars(y)))
                    .to_bracket_geometry_point(),
                &item_name.name.to_string(),
                text_colorpair,
            );
            y += 1;
        }
    }

    batch
        .submit(Consoles::UIText as usize)
        .expect("Couldn't draw inventory listing");

    render_draw_buffer(ctx).expect("Couldn't render inventory listing");
}
