use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;
use units::{Box2DI32, HeightI32, Position2DI32};

use crate::ui::camera::{get_camera_bounds_in_world, render_camera};
use crate::ui::common::Consoles;
use crate::ui::messages::render_messages;
use crate::ui::stats::render_stats;
use crate::ui::tooltips::render_tooltips;
use crate::ui::units::ScreenChars;
use crate::ui::utils::{get_mouse_position, window_size};

pub const MESSAGE_BOX_HEIGHT: HeightI32<ScreenChars> = HeightI32(ScreenChars(6));

#[derive(Debug)]
pub struct PlayerInWorldView {
    pub camera_view: Box2DI32<ScreenChars>,
    pub message_log_view: Box2DI32<ScreenChars>,
    pub stats_view: Box2DI32<ScreenChars>,
    pub window: Box2DI32<ScreenChars>,
    pub mouse_position: Position2DI32<ScreenChars>,
}

impl PlayerInWorldView {
    pub fn from_context(ctx: &mut BTerm) -> PlayerInWorldView {
        let window = ScreenChars::new_box2d_from_size(window_size(ctx));
        let (camera_view, bottom_hud) = window.split_from_bottom(MESSAGE_BOX_HEIGHT);
        let (message_log_view, stats_view) = bottom_hud.split_from_left(bottom_hud.width() / 2);

        PlayerInWorldView {
            camera_view: camera_view,
            message_log_view: message_log_view,
            stats_view: stats_view,
            window: window,
            mouse_position: get_mouse_position(ctx),
        }
    }
}

pub fn render_player_in_world_view(ecs: &World, ctx: &mut BTerm) {
    let main_view = PlayerInWorldView::from_context(ctx);
    let maybe_camera_in_world = get_camera_bounds_in_world(ecs, main_view.camera_view);
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
        main_view.mouse_pt_in_text_chars,
    );

    render_draw_buffer(ctx).expect("Couldn't render camera");
}
