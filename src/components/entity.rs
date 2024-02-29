use std::fmt::{Display, Formatter, Pointer};

use bevy::prelude::Component;
use bevy::utils::Uuid;

/// Represents a Character in the game
#[derive(Component)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub level: u8
}

#[derive(Component)]
pub struct PartyMember;

pub struct HealthModifier {
    pub id: Uuid,
    pub amount: u16
}

#[derive(Component)]
pub struct Health {
    hp: u16,
    temp: u16,
    pub modifiers: Vec<HealthModifier>,
}

impl Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Health: ({} + {})/{}", self.hp, self.temp, self.get_max())
    }
}

impl Health {
    pub fn get_max(&self) -> u16 {
        self.modifiers.iter().map(|x| x.amount).sum()
    }
    pub fn get_current(&self) -> u16 {
        self.hp + self.temp
    }
    pub fn get_temp(&self) -> u16 {
        self.temp
    }
    pub fn get_hp(&self) -> u16 {
        self.hp
    }
    pub fn modify(&mut self, by: i16) {
        let max = self.get_max();
        if by >= 0 {
            let by = by as u16;
            self.hp += by;
            if self.hp > max {
                self.hp = max;
            }
        } else {
            let by = (-by) as u16;
            if self.temp >= by {
                self.temp -= by;
            } else {
                let overflow = by - self.temp;
                if overflow > self.hp {
                    self.hp = 0;
                } else {
                    self.hp -= overflow;
                }
            }
        }
    }
    pub fn set(&mut self, to: u16) {
        self.hp = to;
    }

    pub fn is_dead(&self) -> bool {
        self.get_hp() <= 0
    }

    pub fn new() -> Health {
        Health {
            hp: 0,
            temp: 0,
            modifiers: Vec::new()
        }
    }
}

pub struct Effect {
    pub id: Uuid,
    pub duration: u128,
    pub action: fn(&mut Health) -> ()
}

#[derive(Component)]
pub struct Effects(pub Vec<Effect>);

#[derive(Component)]
pub struct GatedCamera;