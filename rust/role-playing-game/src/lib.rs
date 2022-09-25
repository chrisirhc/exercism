// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => match self.level {
                0..=9 => Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                }),
                _ => Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                }),
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if mana_cost > self.health {
                    self.health = 0
                } else {
                    self.health -= mana_cost;
                }
                0
            }
            Some(mana) if mana < mana_cost => {
                0
            }
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            }
        }
    }
}
