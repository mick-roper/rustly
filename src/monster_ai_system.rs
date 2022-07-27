use super::{Map, Monster, Name, Position, Viewshed};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  #[allow(clippy::type_complexity)]
  type SystemData = (
    WriteExpect<'a, Map>,
    WriteStorage<'a, Viewshed>,
    ReadExpect<'a, Point>,
    ReadStorage<'a, Monster>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, Position>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, mut viewshed, player_pos, monster, named, mut pos) = data;

    for (mut viewshed, _monster, named, mut pos) in
      (&mut viewshed, &monster, &named, &mut pos).join()
    {
      if viewshed.visible_tiles.contains(&*player_pos) {
        let distance =
          rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
        if distance < 1.5 {
          // Attack goes here
          console::log(&format!("{} shouts insults", named.name));
          return;
        }

        let path = rltk::a_star_search(
          map.xy_idx(pos.x, pos.y) as i32,
          map.xy_idx(player_pos.x, player_pos.y) as i32,
          &mut *map,
        );

        if path.success && path.steps.len() > 1 {
          pos.x = path.steps[1] as i32 % map.width;
          pos.y = path.steps[1] as i32 / map.width;
          viewshed.dirty = true;
        }
      }
    }
  }
}
