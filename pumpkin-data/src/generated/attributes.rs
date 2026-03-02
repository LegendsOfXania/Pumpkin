/* This file is generated. Do not edit manually. */
use std::hash::Hash;
#[derive(Clone, Debug)]
pub struct Attributes {
    pub id: u8,
    pub name: &'static str,
    pub default_value: f64,
}
impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Attributes {}
impl Hash for Attributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Attributes {
    pub const ARMOR: Self = Self {
        id: 0,
        name: "armor",
        default_value: 0f64,
    };
    pub const ARMOR_TOUGHNESS: Self = Self {
        id: 1,
        name: "armor_toughness",
        default_value: 0f64,
    };
    pub const ATTACK_DAMAGE: Self = Self {
        id: 2,
        name: "attack_damage",
        default_value: 2f64,
    };
    pub const ATTACK_KNOCKBACK: Self = Self {
        id: 3,
        name: "attack_knockback",
        default_value: 0f64,
    };
    pub const ATTACK_SPEED: Self = Self {
        id: 4,
        name: "attack_speed",
        default_value: 4f64,
    };
    pub const BLOCK_BREAK_SPEED: Self = Self {
        id: 5,
        name: "block_break_speed",
        default_value: 1f64,
    };
    pub const BLOCK_INTERACTION_RANGE: Self = Self {
        id: 6,
        name: "block_interaction_range",
        default_value: 4.5f64,
    };
    pub const BURNING_TIME: Self = Self {
        id: 7,
        name: "burning_time",
        default_value: 1f64,
    };
    pub const CAMERA_DISTANCE: Self = Self {
        id: 8,
        name: "camera_distance",
        default_value: 4f64,
    };
    pub const EXPLOSION_KNOCKBACK_RESISTANCE: Self = Self {
        id: 9,
        name: "explosion_knockback_resistance",
        default_value: 0f64,
    };
    pub const ENTITY_INTERACTION_RANGE: Self = Self {
        id: 10,
        name: "entity_interaction_range",
        default_value: 3f64,
    };
    pub const FALL_DAMAGE_MULTIPLIER: Self = Self {
        id: 11,
        name: "fall_damage_multiplier",
        default_value: 1f64,
    };
    pub const FLYING_SPEED: Self = Self {
        id: 12,
        name: "flying_speed",
        default_value: 0.4f64,
    };
    pub const FOLLOW_RANGE: Self = Self {
        id: 13,
        name: "follow_range",
        default_value: 32f64,
    };
    pub const GRAVITY: Self = Self {
        id: 14,
        name: "gravity",
        default_value: 0.08f64,
    };
    pub const JUMP_STRENGTH: Self = Self {
        id: 15,
        name: "jump_strength",
        default_value: 0.41999998688697815f64,
    };
    pub const KNOCKBACK_RESISTANCE: Self = Self {
        id: 16,
        name: "knockback_resistance",
        default_value: 0f64,
    };
    pub const LUCK: Self = Self {
        id: 17,
        name: "luck",
        default_value: 0f64,
    };
    pub const MAX_ABSORPTION: Self = Self {
        id: 18,
        name: "max_absorption",
        default_value: 0f64,
    };
    pub const MAX_HEALTH: Self = Self {
        id: 19,
        name: "max_health",
        default_value: 20f64,
    };
    pub const MINING_EFFICIENCY: Self = Self {
        id: 20,
        name: "mining_efficiency",
        default_value: 0f64,
    };
    pub const MOVEMENT_EFFICIENCY: Self = Self {
        id: 21,
        name: "movement_efficiency",
        default_value: 0f64,
    };
    pub const MOVEMENT_SPEED: Self = Self {
        id: 22,
        name: "movement_speed",
        default_value: 0.7f64,
    };
    pub const OXYGEN_BONUS: Self = Self {
        id: 23,
        name: "oxygen_bonus",
        default_value: 0f64,
    };
    pub const SAFE_FALL_DISTANCE: Self = Self {
        id: 24,
        name: "safe_fall_distance",
        default_value: 3f64,
    };
    pub const SCALE: Self = Self {
        id: 25,
        name: "scale",
        default_value: 1f64,
    };
    pub const SNEAKING_SPEED: Self = Self {
        id: 26,
        name: "sneaking_speed",
        default_value: 0.3f64,
    };
    pub const SPAWN_REINFORCEMENTS: Self = Self {
        id: 27,
        name: "spawn_reinforcements",
        default_value: 0f64,
    };
    pub const STEP_HEIGHT: Self = Self {
        id: 28,
        name: "step_height",
        default_value: 0.6f64,
    };
    pub const SUBMERGED_MINING_SPEED: Self = Self {
        id: 29,
        name: "submerged_mining_speed",
        default_value: 0.2f64,
    };
    pub const SWEEPING_DAMAGE_RATIO: Self = Self {
        id: 30,
        name: "sweeping_damage_ratio",
        default_value: 0f64,
    };
    pub const TEMPT_RANGE: Self = Self {
        id: 31,
        name: "tempt_range",
        default_value: 10f64,
    };
    pub const WATER_MOVEMENT_EFFICIENCY: Self = Self {
        id: 32,
        name: "water_movement_efficiency",
        default_value: 0f64,
    };
    pub const WAYPOINT_TRANSMIT_RANGE: Self = Self {
        id: 33,
        name: "waypoint_transmit_range",
        default_value: 0f64,
    };
    pub const WAYPOINT_RECEIVE_RANGE: Self = Self {
        id: 34,
        name: "waypoint_receive_range",
        default_value: 0f64,
    };
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "armor" => Some(Self::ARMOR),
            "armor_toughness" => Some(Self::ARMOR_TOUGHNESS),
            "attack_damage" => Some(Self::ATTACK_DAMAGE),
            "attack_knockback" => Some(Self::ATTACK_KNOCKBACK),
            "attack_speed" => Some(Self::ATTACK_SPEED),
            "block_break_speed" => Some(Self::BLOCK_BREAK_SPEED),
            "block_interaction_range" => Some(Self::BLOCK_INTERACTION_RANGE),
            "burning_time" => Some(Self::BURNING_TIME),
            "camera_distance" => Some(Self::CAMERA_DISTANCE),
            "explosion_knockback_resistance" => Some(Self::EXPLOSION_KNOCKBACK_RESISTANCE),
            "entity_interaction_range" => Some(Self::ENTITY_INTERACTION_RANGE),
            "fall_damage_multiplier" => Some(Self::FALL_DAMAGE_MULTIPLIER),
            "flying_speed" => Some(Self::FLYING_SPEED),
            "follow_range" => Some(Self::FOLLOW_RANGE),
            "gravity" => Some(Self::GRAVITY),
            "jump_strength" => Some(Self::JUMP_STRENGTH),
            "knockback_resistance" => Some(Self::KNOCKBACK_RESISTANCE),
            "luck" => Some(Self::LUCK),
            "max_absorption" => Some(Self::MAX_ABSORPTION),
            "max_health" => Some(Self::MAX_HEALTH),
            "mining_efficiency" => Some(Self::MINING_EFFICIENCY),
            "movement_efficiency" => Some(Self::MOVEMENT_EFFICIENCY),
            "movement_speed" => Some(Self::MOVEMENT_SPEED),
            "oxygen_bonus" => Some(Self::OXYGEN_BONUS),
            "safe_fall_distance" => Some(Self::SAFE_FALL_DISTANCE),
            "scale" => Some(Self::SCALE),
            "sneaking_speed" => Some(Self::SNEAKING_SPEED),
            "spawn_reinforcements" => Some(Self::SPAWN_REINFORCEMENTS),
            "step_height" => Some(Self::STEP_HEIGHT),
            "submerged_mining_speed" => Some(Self::SUBMERGED_MINING_SPEED),
            "sweeping_damage_ratio" => Some(Self::SWEEPING_DAMAGE_RATIO),
            "tempt_range" => Some(Self::TEMPT_RANGE),
            "water_movement_efficiency" => Some(Self::WATER_MOVEMENT_EFFICIENCY),
            "waypoint_transmit_range" => Some(Self::WAYPOINT_TRANSMIT_RANGE),
            "waypoint_receive_range" => Some(Self::WAYPOINT_RECEIVE_RANGE),
            _ => None,
        }
    }
}
