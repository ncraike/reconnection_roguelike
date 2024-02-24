use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;

use super::super::components::{CombatStats, Player};
use super::colors;
use super::common::Consoles;

const HEALTH_CRITICAL_COLOR: RGB = colors::BRIGHT_ORANGE;
const HEALTH_OKAY_COLOR: RGB = colors::BRIGHT_YELLOW;
const HEALTH_GOOD_COLOR: RGB = colors::GREEN_MID;

pub fn render_stats(ecs: &World, batch: &mut DrawBatch, bounds: Rect) {
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
        .submit(Consoles::UIText as usize)
        .expect("Couldn't render player stats");
}
