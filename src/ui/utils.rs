use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{to_cp437, DrawBatch};

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
