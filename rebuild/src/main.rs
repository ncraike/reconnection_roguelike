use bracket_geometry::prelude::Point;
use bracket_lib::prelude::main_loop;
use bracket_terminal;
use bracket_terminal::prelude::{BError, EMBED};

use specs::prelude::*;

use reconnection_roguelike::components::register_components;
use reconnection_roguelike::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use reconnection_roguelike::message_log::MessageLog;
use reconnection_roguelike::types::RunState;
use reconnection_roguelike::ui::common::UIState; // FIXME: move to UI setup
use reconnection_roguelike::ui::keyboard::{classic_laptop, Keybindings};
use reconnection_roguelike::ui::UI;
use reconnection_roguelike::world::engine::WorldEngineState;
use reconnection_roguelike::world::spawner::{
    create_bandage, create_enemy_big_stalker, create_enemy_hound, create_first_aid_kit,
    create_player,
};
use reconnection_roguelike::world::WorldEngine; // FIXME: move to WorldEngine setup

use reconnection_roguelike::State;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

    let mut gs = State { ecs: World::new() };
    register_components(&mut gs.ecs);

    let ui: UI = UI {};
    gs.ecs.insert::<UI>(UI {});
    gs.ecs.insert::<UIState>(UIState::PlayerInWorld); // FIXME: move to UI setup
    gs.ecs.insert::<Keybindings>(classic_laptop()); // FIXME: move to UI setup

    let ctx = ui.build_terminal()?;

    let world_engine: WorldEngine = WorldEngine {};
    gs.ecs.insert::<WorldEngine>(world_engine);
    gs.ecs
        .insert::<WorldEngineState>(WorldEngineState::PlayerTurn); // FIXME: move to WorldEngine setup

    gs.ecs.insert(RunState::PreRun);

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    gs.ecs.insert(MessageLog { entries: vec![] });

    let player = create_player(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2) as i32,
            y: (MAP_HEIGHT / 2) as i32,
        },
    );
    gs.ecs.insert(player);
    create_enemy_hound(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 4) as i32,
        },
    );
    create_enemy_big_stalker(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_bandage(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 - MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_bandage(
        &mut gs.ecs,
        Point {
            x: ((MAP_WIDTH / 2 - MAP_WIDTH / 4) + 1) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_first_aid_kit(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 - MAP_WIDTH / 4) as i32,
            y: ((MAP_HEIGHT / 2 + MAP_HEIGHT / 4) + 1) as i32,
        },
    );

    main_loop(ctx, gs)
}
