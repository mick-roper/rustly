use rltk::{RandomNumberGenerator, DiceType};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
  Wall,
  Floor
}

pub struct Map {
  w: i32,
  h: i32,
  tiles: Vec<TileType>
}

struct Room {
  x: i32,
  y: i32,
  w: i32,
  h: i32,
}

impl Map {
  pub fn new(w: i32, h: i32) -> Result<Map, String> {
    let tiles = vec![TileType::Floor; (w * h) as usize];
    let mut rng = RandomNumberGenerator::new();

    const max_width: i32 = 20;
    const max_height: i32 = 20;

    let mut rooms = Vec::new();

    // draw some rooms
    let mut counter = 0;
    while counter < 10 {
      let room = Room {
        x: rng.range(5, w - max_width*2),
        y: rng.range(5, h - max_height*2),
        w: rng.range(5, max_width),
        h: rng.range(5, max_height),
      };

      for r in rooms.iter() {
        if room.intersects(&r) {
          continue
        }
      }

      rooms.push(room);

      counter+=1;
    }

    Ok(Map{ w, h, tiles })
  }
}

impl Room {
  fn intersects(&self, other: &Room) -> bool {
    (self.x >= other.x && other.x <= self.x && self.y >= other.y && other.y <= self.y) ||
    (self.w >= other.w && other.w <= self.w && self.h >= other.h && other.h <= self.h)
  }
}