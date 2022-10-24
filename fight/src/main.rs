use std::cmp::Reverse;

use liso::{IO, liso, Response};

struct Person {
    hp: u32,
    potions: u32,
}

fn main() {
    let mut me = Person {
        hp: 50,
        potions: 3,
    };

    let mut enemy = Person {
        hp: 100,
        potions: 0,
    };

    let mut io = IO::new();
    io.prompt(liso!(fg=green, +bold, "> ", reset), true, true);

    while me.hp != 0 && enemy.hp != 0 {
        let mut banner = "-------------\n\
        | att | pot |\n\
        -------------";

        io.status(Some(liso!(fg=green, format!("HP: {}/{} Pot: {}    Enemy HP: {}/{}\n{}", me.hp, 50, me.potions,enemy.hp, 100,banner))));

        let input = match io.blocking_read() {
            Response::Input(line) => match line.as_str() {
                "attack" | "att" => {
                    enemy.hp -= 10;
                    // attack things
                },
                "potion" | "pot" => {
                    if me.potions == 0 {
                        io.println("Out of potions");
                        continue;
                    } else if me.hp >= 20 {
                        me.hp = 50;
                    } else {
                        me.hp += 30;
                    }
                    me.potions -= 1;
                },
                "quit" => {
                    return;
                },
                _ => {
                    io.println("No valid choice made");
                    continue;
                },
            },
            Response::Dead | Response::Quit => return,
            other => {
                io.notice(format!("unknown key {}", other.as_unknown() as char),
                    std::time::Duration::from_secs(3));
            },
        };
        
        io.println("enemy is attacking");
        me.hp -= 10;


    }
    if me.hp == 0 && enemy.hp == 0 {
        io.println("Both of you died fighting for your lives");
    } else if me.hp == 0 {
        io.println("You died fighting...");
    } else {
        io.println("You were able to kill your enemy");
    }
}
