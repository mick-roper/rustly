use super::{Monster, Viewshed, Named};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadStorage<'a, Viewshed>,
    ReadExpect<'a, Point>,
    ReadStorage<'a, Monster>,
    ReadStorage<'a, Named>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (viewshed, player_pos, monster, named) = data;

    for (viewshed, _monster, named) in (&viewshed, &monster, &named).join() {
      if viewshed.visible_tiles.contains(&*player_pos) {
        console::log(&format!("{} shouts insults!", named.name))
      }
    }
  }
}
