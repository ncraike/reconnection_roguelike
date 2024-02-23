use bracket_geometry::prelude::Rect;
use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;

use crate::ui::common::Consoles;
use crate::ui::utils::{get_mouse_point_in_text_chars, get_mouse_point_in_tiles2x, window_size};
use units::{Box2D, Height, Point2D, TextChars, Tiles2x};

use super::super::camera::{get_camera_bounds_in_world, render_camera};
use super::super::messages::render_messages;
use super::super::stats::render_stats;
use super::super::tooltips::render_tooltips;

pub const MESSAGE_BOX_HEIGHT: Height<TextChars> = Height(TextChars(6));

#[derive(Debug)]
pub struct PlayerInWorldView {
    pub camera_view: Box2D<Tiles2x>,
    pub message_log_view: Box2D<TextChars>,
    pub stats_view: Box2D<TextChars>,
    pub window: Box2D<Tiles2x>,
    pub mouse_pt_in_tiles2x: Point2D<Tiles2x>,
    pub mouse_pt_in_text_chars: Point2D<TextChars>,
}

impl PlayerInWorldView {
    pub fn from_context(ctx: &mut BTerm) -> PlayerInWorldView {
        let window = Box2D::new_from_size(window_size(ctx));
        let (camera_view, bottom_hud) =
            window.split_from_bottom(MESSAGE_BOX_HEIGHT.to_tiles2x_ceil());
        let bottom_hud = bottom_hud.to_text_chars_floor();
        let (message_log_view, stats_view) = bottom_hud.split_from_left(bottom_hud.width() / 2);

        PlayerInWorldView {
            camera_view: camera_view,
            message_log_view: message_log_view,
            stats_view: stats_view,
            window: window,
            mouse_pt_in_tiles2x: get_mouse_point_in_tiles2x(ctx),
            mouse_pt_in_text_chars: get_mouse_point_in_text_chars(ctx),
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
