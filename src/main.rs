use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, EMBED};

use specs::prelude::*;

pub mod camera;
pub mod components;
pub mod damage_system;
pub mod gui;
pub mod map;
pub mod map_indexing_system;
pub mod melee_combat_system;
pub mod message_log;
pub mod monster_ai_system;
pub mod player;
pub mod visibility_system;
use components::{
    build_monster_entities, insert_player_entity, register_components, BlocksTile, Player, Point,
    Viewshed,
};
use damage_system::DamageSystem;
use gui::{build_terminal, render_main_view};
use map::Map;
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use message_log::MessageLog;
use monster_ai_system::MonsterAI;
use player::player_input;
use visibility_system::VisibilitySystem;

pub const GAME_TITLE: &str = "Reconnection";

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    ecs: World,
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
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
                // FIXME: this is needed to avoid "jitter" in vision rendering
                let mut vis = VisibilitySystem {};
                vis.run_now(&self.ecs);
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
        }

        {
            let mut run_state_writer = self.ecs.write_resource::<RunState>();
            *run_state_writer = new_run_state;
        }

        damage_system::delete_the_dead(&mut self.ecs);
        render_main_view(&self.ecs, ctx);
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
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

    insert_player_entity(&mut gs.ecs);
    build_monster_entities(&mut gs.ecs);

    main_loop(terminal, gs)
}
