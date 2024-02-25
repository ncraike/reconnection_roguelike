use bracket_color::prelude::{ColorPair, RgbLerp, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;
use units::Box2DI32;

use super::super::message_log::MessageLog;

use super::colors;
use super::common::Consoles;
use super::units::ScreenChars;

const FIRST_MESSAGE_COLOR: RGB = colors::BLUE_LIGHT;
const LAST_MESSAGE_COLOR: RGB = colors::BLUE_MID_DARK;

pub fn render_messages(ecs: &World, batch: &mut DrawBatch, bounds: Box2DI32<ScreenChars>) {
    let message_log = ecs.fetch::<MessageLog>();
    let tail = message_log
        .entries
        .iter()
        .rev()
        .take(bounds.height().to_primitive() as usize);
    let mut color_lerp = RgbLerp::new(
        FIRST_MESSAGE_COLOR,
        LAST_MESSAGE_COLOR,
        bounds.height().to_primitive() as usize,
    );

    batch.target(Consoles::UIText as usize);
    for (line_no, entry) in tail.enumerate() {
        batch.print_color(
            (bounds.p1 + ScreenChars::new_height(line_no as i32)).to_bracket_geometry_point(),
            entry,
            ColorPair {
                fg: color_lerp.next().unwrap_or(LAST_MESSAGE_COLOR).to_rgba(1.0),
                bg: colors::TRANSPARENT,
            },
        );
    }
    batch
        .submit(Consoles::UIText as usize)
        .expect("Couldn't render message log");
}
