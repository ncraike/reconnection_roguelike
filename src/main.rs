use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, BTermBuilder, EMBED};

use specs::prelude::*;

pub mod camera;
pub mod components;
pub mod damage_system;
pub mod map;
pub mod map_indexing_system;
pub mod melee_combat_system;
pub mod monster_ai_system;
pub mod player;
pub mod visibility_system;
use camera::{render_camera, DEFAULT_VIEW_HEIGHT, DEFAULT_VIEW_WIDTH};
use components::{
    BlocksTile, CombatStats, Monster, Name, Player, Point, Renderable, SufferDamage, Viewshed,
    WantsToMelee,
};
use damage_system::DamageSystem;
use map::{Map, TileGraphic, MAP_HEIGHT, MAP_WIDTH, TILE_2X_HEIGHT, TILE_2X_WIDTH};
use map_indexing_system::MapIndexingSystem;
use melee_combat_system::MeleeCombatSystem;
use monster_ai_system::MonsterAI;
use player::player_input;
use visibility_system::VisibilitySystem;

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
        render_camera(&self.ecs, ctx);
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

    let context = BTermBuilder::new()
        .with_automatic_console_resize(true)
        .with_fitscreen(true)
        .with_font(
            "reconnection_16x24_tiles_at_2x.png",
            TILE_2X_WIDTH,
            TILE_2X_HEIGHT,
        )
        .with_font("vga8x16.png", 8, 16)
        .with_simple_console(
            DEFAULT_VIEW_WIDTH,
            DEFAULT_VIEW_HEIGHT,
            "reconnection_16x24_tiles_at_2x.png",
        )
        .with_sparse_console_no_bg(
            DEFAULT_VIEW_WIDTH,
            DEFAULT_VIEW_HEIGHT,
            "reconnection_16x24_tiles_at_2x.png",
        )
        .with_sparse_console_no_bg(
            DEFAULT_VIEW_WIDTH * (TILE_2X_WIDTH / 8),
            DEFAULT_VIEW_HEIGHT * (TILE_2X_HEIGHT / 16),
            "vga8x16.png",
        )
        .with_title("Reconnection")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Point>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<WantsToMelee>();

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    gs.ecs.insert(RunState::PreRun);

    let player_entity = gs
        .ecs
        .create_entity()
        .with(Player {})
        .with(Name {
            name: String::from("Player"),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .with(Point {
            x: (MAP_WIDTH / 2) as i32,
            y: (MAP_HEIGHT / 2) as i32,
        })
        .with(Renderable {
            graphic: TileGraphic::PlayerCharacter,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();
    gs.ecs.insert(player_entity);

    gs.ecs
        .create_entity()
        .with(Monster {})
        .with(Name {
            name: String::from("H-32"),
        })
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .with(BlocksTile {})
        .with(Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 4) as i32,
        })
        .with(Renderable {
            graphic: TileGraphic::EnemyHound,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();
    gs.ecs
        .create_entity()
        .with(Monster {})
        .with(Name {
            name: String::from("S-07"),
        })
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .with(BlocksTile {})
        .with(Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        })
        .with(Renderable {
            graphic: TileGraphic::EnemyBigStalker,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    main_loop(context, gs)
}
