use bracket_color::prelude::{ColorPair, RGB};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;
use units::Box2DI32;

use super::super::components::{CombatStats, Player};
use super::colors;
use super::common::Consoles;
use super::units::ScreenChars;

const HEALTH_CRITICAL_COLOR: RGB = colors::BRIGHT_ORANGE;
const HEALTH_OKAY_COLOR: RGB = colors::BRIGHT_YELLOW;
const HEALTH_GOOD_COLOR: RGB = colors::GREEN_MID;

pub fn render_stats(ecs: &World, batch: &mut DrawBatch, bounds: Box2DI32<ScreenChars>) {
    let stats_store = ecs.read_storage::<CombatStats>();
    let player_store = ecs.read_storage::<Player>();

    batch.target(Consoles::UIText as usize);

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
            bg: colors::TRANSPARENT,
        };
        batch.print_color(
            (bounds.p1 + ScreenChars::new_height(1)).to_bracket_geometry_point(),
            &health_text,
            health_color,
        );
        let health_bar_offset = ScreenChars::new_width(health_text.len() as i32 + 4);
        let health_bar_width = bounds.width() - health_bar_offset - ScreenChars::new_width(4);
        if health_bar_width > ScreenChars::new_width(0) {
            batch.bar_horizontal(
                (bounds.p1 + health_bar_offset + ScreenChars::new_height(1))
                    .to_bracket_geometry_point(),
                health_bar_width.to_primitive(),
                stats.hp,
                stats.max_hp,
                health_color,
            );
        }
    }

    batch
        .submit(Consoles::UIText as usize)
        .expect("Couldn't render player stats");
}
