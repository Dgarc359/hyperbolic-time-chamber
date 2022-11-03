use std::collections::HashMap;

type MyMap<'a> = HashMap<u16, &'a u8>;

pub struct Board<'a> {
  pub position: MyMap<'a>
}

fn build_board() -> MyMap<'static> {
  let mut map: MyMap = HashMap::new();

  map.insert(0, &7);
  map
}

impl Board<'_> {
  pub fn new() -> Self {
    let pos: MyMap = build_board();
    Board {
      position: pos
    }
  }

  pub fn move_piece(&mut self, from: u16, to: u16){
    let piece = self.position.get(&from).unwrap().to_owned();
    self.position.remove(&from);
    self.position.insert(to, piece);
  }
}