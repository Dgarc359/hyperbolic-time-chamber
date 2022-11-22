mod board;
pub use board::{Board, Team, Pieces};

use std::cmp::Reverse;
use liso::{IO, liso, Response};

pub struct ChessController {
  board: Board,
}

impl ChessController {
  pub fn new() -> Self {
    Self {
      board: Board::new(),
    }
  }

  pub fn start_game(&self) {
    // todo!("Game loop to be created");
    let mut io = IO::new();
    io.prompt(liso!(fg=green, +bold, "> ", reset), true, true);
    while true {


      io.status(Some(liso!(fg=green, format!("8  rkbQKbkr\n7  pppppppp\n6  --------\n5  --------\n4  --------\n3  --------\n2  pppppppp\n1  rkbQKbkr\n\n   abcdefgh"))));

      let input = match io.blocking_read() {
        Response::Input(line) => match line.as_str() {
            "attack" | "att" | "a" => {
                // let damage = me.attack_target(&mut enemy);
                // if damage > 0 {
                //     io.println(format!("You attack your target, dealing {} damage", damage));
                // }
            },
            "potion" | "pot" | "p" => {
                // match me.try_use_potion() {
                //     Ok(HealResult::Good { hp_gained }) => {
                //         io.wrapln(format!("You used your potion to heal {} and now you're at {} hp", hp_gained, me.hp));
                //     },
                //     Ok(HealResult::Wasteful { hp_gained, hp_wasted }) => {
                //         io.wrapln(format!("You used your potion, but unfortunately you wasted {} of it's regenerative properties... You still healed {} to get to full health though!", hp_wasted, hp_gained));
                //     },
                //     Err(_) => {
                //         // io.wrapln("Out of potions");
                //         io.wrapln("Failed to drink a potion");
                //         continue;
                //     },
                // }
            },
            "quit" => {
                return;
            },
            _ => {
                io.wrapln("No valid choice made");
                continue;
            },
        },
        Response::Dead | Response::Quit => return,
        other => {
            io.notice(format!("unknown key {}", other.as_unknown() as char),
                std::time::Duration::from_secs(3));
        },
    };
    }
    // while loop
  }
}