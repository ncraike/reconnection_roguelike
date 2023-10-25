use crate::ui::common::Consoles;
use crate::ui::units::{Point2D, Size2D, TextChars, Tiles2x};
use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{to_cp437, BTerm, DrawBatch};

pub fn window_size(ctx: &mut BTerm) -> Size2D<Tiles2x> {
    ctx.set_active_console(Consoles::TilesTerrain as usize);
    let (width_in_tiles, height_in_tiles) = ctx.get_char_size();
    Tiles2x::new_size2d(width_in_tiles as i32, height_in_tiles as i32)
}

pub fn get_mouse_point_in_tiles2x(ctx: &mut BTerm) -> Point2D<Tiles2x> {
    ctx.set_active_console(Consoles::TilesTerrain as usize);
    let bracket_point = ctx.mouse_point();
    Tiles2x::new_point2d(bracket_point.x, bracket_point.y)
}

pub fn get_mouse_point_in_text_chars(ctx: &mut BTerm) -> Point2D<TextChars> {
    ctx.set_active_console(Consoles::Text as usize);
    let bracket_point = ctx.mouse_point();
    TextChars::new_point2d(bracket_point.x, bracket_point.y)
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
