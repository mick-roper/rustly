use specs::prelude::*;
use super::{CombatStats, SufferDamage};

pub struct DamageSystem{}

impl<'a> System<'a> for DamageSystem {
  type SystemData = (
    WriteStorage<'a, CombatStats>,
    WriteStorage<'a, SufferDamage>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut combat_stats, mut damage) = data;

    for (stats, damage) in (&mut combat_stats, &damage).join() {
      stats.current_hp -= damage.amount.iter().sum::<i32>();
    }

    damage.clear();
  }
}