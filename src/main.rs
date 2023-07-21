use bracket_geometry::prelude::Point;
use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, EMBED};

use specs::prelude::*;

pub mod components;
pub mod map;
pub mod message_log;
pub mod player;
pub mod ui;
pub mod world;

use components::register_components;
use map::{Map, MAP_HEIGHT, MAP_WIDTH};
use message_log::MessageLog;
use player::{player_input, player_input_inventory_menu};
use ui::common::build_terminal;
use ui::main_view::render_main_view;
use ui::menus::render_inventory_menu;
use world::spawner::{
    create_bandage, create_enemy_big_stalker, create_enemy_hound, create_first_aid_kit,
    create_player,
};
use world::systems;
use world::systems::damage::delete_the_dead;
use world::systems::map_indexing::MapIndexingSystem;
use world::systems::visibility::VisibilitySystem;

pub const GAME_TITLE: &str = "Reconnection";

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ActiveMenu(Menu),
}

#[derive(PartialEq, Copy, Clone)]
pub enum Menu {
    Inventory(InventoryMenuState),
    Stats,
    Skills,
}

#[derive(PartialEq, Copy, Clone)]
pub enum InventoryMenuState {
    AwaitingInput,
    UseItem,
}

pub struct State {
    ecs: World,
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut new_run_state;
        {
            let run_state = self.ecs.fetch::<RunState>();
            new_run_state = *run_state;
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_run_state = player_input(self, ctx);
                // FIXME: fix "jitter" in vision rendering
                let mut vis = VisibilitySystem {};
                vis.run_now(&self.ecs);
                // FIXME: fix out-of-date monster positions for tooltips
                let mut map_index = MapIndexingSystem {};
                map_index.run_now(&self.ecs);
                self.ecs.maintain();
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::ActiveMenu(menu) => match menu {
                Menu::Inventory(state) => match state {
                    InventoryMenuState::AwaitingInput => {
                        new_run_state = player_input_inventory_menu(ctx);
                    }
                    InventoryMenuState::UseItem => {
                        // FIXME: implement using items
                    }
                },
                // FIXME: implement other options
                _ => {}
            },
        }

        {
            let mut run_state_writer = self.ecs.write_resource::<RunState>();
            *run_state_writer = new_run_state;
        }

        match new_run_state {
            RunState::ActiveMenu(menu) => match menu {
                Menu::Inventory(menu_state) => {
                    render_inventory_menu(&self.ecs, ctx, menu_state);
                }
                _ => {}
            },
            _ => {
                delete_the_dead(&mut self.ecs);
                render_main_view(&self.ecs, ctx);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        systems::run(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

    let terminal: BTerm = build_terminal()?;

    let mut gs = State { ecs: World::new() };
    register_components(&mut gs.ecs);

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

    main_loop(terminal, gs)
}
