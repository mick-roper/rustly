use super::{Map, Monster, Position, Viewshed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Monster>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (viewshed, pos, monster) = data;

    for (_viewshed, _pos, _monster) in (&viewshed, &pos, &monster).join() {
      console::log("monster considers their own existence...")
    }
  }
}
