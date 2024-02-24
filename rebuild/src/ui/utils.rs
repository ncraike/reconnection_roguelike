use bracket_color::prelude::ColorPair;
use bracket_terminal::prelude::{to_cp437, BTerm, DrawBatch};
use units::{Box2DI32, Position2DI32, Size2DI32, WidthI32};

use crate::ui::common::Consoles;
use crate::ui::units::ScreenChars;

pub fn window_size(ctx: &mut BTerm) -> Size2DI32<ScreenChars> {
    ctx.set_active_console(Consoles::WorldTerrain as usize);
    let (width, height) = ctx.get_char_size();
    ScreenChars::new_size2d(width as i32, height as i32)
}

pub fn get_mouse_position(ctx: &mut BTerm) -> Position2DI32<ScreenChars> {
    ctx.set_active_console(Consoles::WorldTerrain as usize);
    let bracket_point = ctx.mouse_point();
    ScreenChars::new_position2d(bracket_point.x, bracket_point.y)
}

pub fn draw_box_with_filled_bg(
    batch: &mut DrawBatch,
    bounds: Box2DI32<ScreenChars>,
    colorpair: ColorPair,
) {
    let fill_bounds = Box2DI32 {
        p1: bounds.p1,
        p2: bounds.p2 + ScreenChars::new_size2d(1, 1),
    };
    batch.fill_region(
        fill_bounds.to_bracket_geometry_rect(),
        ColorPair::new(colorpair.bg, colorpair.bg),
        to_cp437('█'),
    );
    batch.draw_box(bounds.to_bracket_geometry_rect(), colorpair);
}

pub fn print_color_with_filled_bg<S: ToString>(
    batch: &mut DrawBatch,
    position: Position2DI32<ScreenChars>,
    text: S,
    colorpair: ColorPair,
    left_padding: WidthI32<ScreenChars>,
    right_padding: WidthI32<ScreenChars>,
) {
    let as_string = text.to_string();
    // FIXME: this will be wrong for multi-byte characters
    let width = ScreenChars::new_width(as_string.len() as i32);
    let fill_bounds = ScreenChars::new_box2d_from_position_and_size(
        position - left_padding,
        Size2DI32 {
            width: width + right_padding + ScreenChars::new_width(1),
            height: ScreenChars::new_height(1),
        },
    );
    // FIXME: assert this is still in the screen space
    batch.fill_region(
        fill_bounds.to_bracket_geometry_rect(),
        ColorPair::new(colorpair.bg, colorpair.bg),
        to_cp437('█'),
    );
    batch.print_color(position.to_bracket_geometry_point(), text, colorpair);
}
