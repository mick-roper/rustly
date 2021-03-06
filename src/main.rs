use rltk::{GameState, Rltk, RGB, Point};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_systems;
use visibility_systems::VisibilitySystem;
mod monster_ai_system;
pub use monster_ai_system::*;
mod map_indexing_system;
pub use map_indexing_system::*;
mod melee_combat_system;
pub use melee_combat_system::*;
mod damage_system;
pub use damage_system::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Paused,
}

pub struct State {
    pub ecs: World,
    pub run_state: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut ai = MonsterAI {};
        ai.run_now(&self.ecs);

        let mut map_indexer = MapIndexingSystem{};
        map_indexer.run_now(&self.ecs);

        let mut melee_combat_system = MeleeCombatSystem{};
        melee_combat_system.run_now(&self.ecs);

        let mut dmg_system = DamageSystem{};
        dmg_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_systems();
            delete_the_dead(&mut self.ecs);
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }


        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> rltk::BError {
    let mut rng = rltk::RandomNumberGenerator::new();

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Rustly").build()?;

    let mut gs = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map = Map::new(&mut rng);

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {glyph = rltk::to_cp437('g'); name = "Goblin".to_string();},
            _ => {glyph = rltk::to_cp437('o'); name= "Orc".to_string();},
        }

        gs.ecs
            .create_entity()
            .with(Monster {})
            .with(Name{ name: format!("{} #{}", name , i) })
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                range: 8,
                visible_tiles: Vec::new(),
                dirty: true,
            })
            .with(BlocksTile{})
            .with(CombatStats{
                max_hp: 16,
                current_hp: 16,
                defence: 1,
                power: 2
            })
            .build();
    }

    let (player_x, player_y) = map.start_pos;
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats{
                max_hp: 30,
                current_hp: 30,
                defence: 2,
                power: 5
            })
        .build();

    rltk::main_loop(context, gs)
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let floor_fg = RGB::from_f32(0.5, 0.5, 0.5);
    let wall_fg = RGB::from_f32(0., 1., 0.);
    let bg = RGB::from_f32(0., 0., 0.);
    let floor_glyph = rltk::to_cp437('.');
    let wall_glyph = rltk::to_cp437('#');

    let mut x = 0;
    let mut y = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (mut fg, glyph) = match tile {
            TileType::Floor => (floor_fg, floor_glyph),
            TileType::Wall => (wall_fg, wall_glyph),
        };

        if map.revealed_tiles[idx] {
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale();
            }
            ctx.set(x, y, fg, bg, glyph);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.current_hp < 1 {
                dead.push(entity);
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("could not delete entity")
    }
}
