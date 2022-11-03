use std::collections::HashMap;

type myMap<'a> = HashMap<u16, &'a u8>;

pub struct Board<'a> {
  pub position: myMap<'a>
}

fn build_board() -> myMap<'static> {
  let mut map: myMap = HashMap::new();

  map.insert(0, &7);
  map
}

impl Board<'_> {
  pub fn new() -> Self {
    let pos: myMap = build_board();
    Board {
      position: pos
    }
  }

  pub fn move_piece(&mut self, from: u16, to: u16){
    let piece = self.position.get(&from).unwrap();
    self.position.remove(&from);
    self.position.insert(to, piece);
  }
}