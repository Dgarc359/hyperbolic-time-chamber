use std::cmp::Reverse;

use liso::{IO, liso, Response};

const PLAYER_MAX_HP: u32 = 50;
const PLAYER_STARTING_POTION: u32 = 3;
const ENEMY_MAX_HP: u32 = 321;
const ENEMY_STARTING_POTION: u32 = 0;
const POTION_HEAL_AMOUNT: u32 = 30;
const PLAYER_ATTACK_DAMAGE: u32 = 33;
const ENEMY_ATTACK_DAMAGE: u32 = 10;

struct Person {
    max_hp: u32,
    hp: u32,
    potions: u32,
    damage: u32,
}

#[derive(Debug, PartialEq)]
enum HealResult {
    Good { hp_gained: u32 },
    Wasteful { hp_gained: u32, hp_wasted: u32 }
}

impl Person {
    fn try_use_potion(&mut self) -> Result<HealResult, ()> {
        if self.potions > 0 {
            self.potions -= 1;
            let old_hp = self.hp;
            let new_hp = (old_hp + POTION_HEAL_AMOUNT).min(self.max_hp);
            self.hp = new_hp;
            let hp_gained = new_hp - old_hp;
            let hp_wasted = POTION_HEAL_AMOUNT - hp_gained;
            if hp_wasted > 0 {
                Ok(HealResult::Wasteful { hp_gained, hp_wasted })
            }
            else {
                assert_eq!(hp_gained, 30);
                Ok(HealResult::Good { hp_gained })
            }
        }
        else {
            Err(())
        }
    }
    fn attack_target(&mut self, target: &mut Person) -> u32 {
        let old_hp = target.hp;
        let new_hp = old_hp.saturating_sub(self.damage);
        target.hp = new_hp;
        old_hp - new_hp
    }
    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

fn main() {
    let mut me = Person {
        max_hp: PLAYER_MAX_HP,
        hp: PLAYER_MAX_HP,
        potions: PLAYER_STARTING_POTION,
        damage: PLAYER_ATTACK_DAMAGE
    };

    let mut enemy = Person {
        max_hp: PLAYER_MAX_HP,
        hp: ENEMY_MAX_HP,
        potions: ENEMY_STARTING_POTION,
        damage: ENEMY_ATTACK_DAMAGE
    };

    let mut io = IO::new();
    io.prompt(liso!(fg=green, +bold, "> ", reset), true, true);

    while me.hp != 0 && enemy.hp != 0 {

        io.status(Some(liso!(fg=green, format!("HP: {}/{} Pot: {}    Enemy HP: {}/{}", me.hp, PLAYER_MAX_HP, me.potions,enemy.hp, ENEMY_MAX_HP))));

        let input = match io.blocking_read() {
            Response::Input(line) => match line.as_str() {
                "attack" | "att" | "a" => {
                    let damage = me.attack_target(&mut enemy);
                    if damage > 0 {
                        io.println(format!("You attack your target, dealing {} damage", damage));
                    }
                },
                "potion" | "pot" | "p" => {
                    match me.try_use_potion() {
                        Ok(HealResult::Good { hp_gained }) => {
                            io.wrapln(format!("You used your potion to heal {} and now you're at {} hp", hp_gained, me.hp));
                        },
                        Ok(HealResult::Wasteful { hp_gained, hp_wasted }) => {
                            io.wrapln(format!("You used your potion, but unfortunately you wasted {} of it's regenerative properties... You still healed {} to get to full health though!", hp_wasted, hp_gained));
                        },
                        Err(_) => {
                            // io.wrapln("Out of potions");
                            io.wrapln("Failed to drink a potion");
                            continue;
                        },
                    }
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
        
        let enemy_damage = enemy.attack_target(&mut me);
        io.wrapln(format!("The enemy is attacking! You have lost {} health", enemy_damage));
    }
    if !me.is_alive() && !enemy.is_alive() {
        io.wrapln("Both of you died fighting for your lives");
    } else if !me.is_alive() {
        io.wrapln("You died fighting...");
    } else {
        io.wrapln("You were able to kill your enemy");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn weird_potion_health() {
        let mut uke = Person {
            max_hp: PLAYER_MAX_HP*2 + POTION_HEAL_AMOUNT,
            hp: PLAYER_MAX_HP*2,
            potions: 1,
            damage: 3,
        };
        assert_eq!(uke.try_use_potion(), Ok(HealResult::Good { hp_gained: POTION_HEAL_AMOUNT }));
    }
}