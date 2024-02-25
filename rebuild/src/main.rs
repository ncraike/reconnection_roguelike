use bracket_lib::prelude::main_loop;
use bracket_terminal;
use bracket_terminal::prelude::BError;

use specs::prelude::*;

use rebuild::components::{register_components, WorldPosition2D};
use rebuild::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use rebuild::message_log::MessageLog;
use rebuild::types::RunState;
use rebuild::ui::common::UIState; // FIXME: move to UI setup
use rebuild::ui::keyboard::{classic_laptop, Keybindings};
use rebuild::ui::UI;
use rebuild::world::engine::WorldEngineState;
use rebuild::world::spawner::{
    create_bandage, create_enemy_big_stalker, create_enemy_hound, create_first_aid_kit,
    create_player,
};
use rebuild::world::WorldEngine; // FIXME: move to WorldEngine setup

use rebuild::State;

fn main() -> BError {
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
        WorldPosition2D {
            x: (MAP_WIDTH / 2) as i32,
            y: (MAP_HEIGHT / 2) as i32,
        },
    );
    gs.ecs.insert(player);
    create_enemy_hound(
        &mut gs.ecs,
        WorldPosition2D {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 4) as i32,
        },
    );
    create_enemy_big_stalker(
        &mut gs.ecs,
        WorldPosition2D {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_bandage(
        &mut gs.ecs,
        WorldPosition2D {
            x: (MAP_WIDTH / 2 - MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_bandage(
        &mut gs.ecs,
        WorldPosition2D {
            x: ((MAP_WIDTH / 2 - MAP_WIDTH / 4) + 1) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_first_aid_kit(
        &mut gs.ecs,
        WorldPosition2D {
            x: (MAP_WIDTH / 2 - MAP_WIDTH / 4) as i32,
            y: ((MAP_HEIGHT / 2 + MAP_HEIGHT / 4) + 1) as i32,
        },
    );

    main_loop(ctx, gs)
}
