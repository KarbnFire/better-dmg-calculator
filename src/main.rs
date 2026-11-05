use std::f32;

use rand::prelude::*;

const MIN_DICE_ROLL: u32 = 1;
const MAX_D4_ROLL: u32 = 4;
const MAX_D6_ROLL: u32 = 6;
const MAX_D8_ROLL: u32 = 8;
const MAX_D10_ROLL: u32 = 10;
const MAX_D12_ROLL: u32 = 12;
const MAX_D20_ROLL: u32 = 20;
const MAX_D100_ROLL: u32 = 100;

fn main() {
    //println!("Roll result is {}", roll_die());
    let enemy_ac: u32 = 16;
    let hit_mod: u32 = 50;

    println!(
        "Hit chance on {enemy_ac} AC {hit_mod} hit mod is {}%",
        (calculate_single_dice_hit_chance(enemy_ac, hit_mod, MIN_DICE_ROLL, MAX_D20_ROLL) * 100.0)
            as u32
    );
}

fn roll_die() -> u32 {
    let roll_result: u32 = rand::rng().random_range(1..=20);

    return roll_result;
}

fn calculate_single_dice_hit_chance(
    enemy_ac: u32,
    mut hit_modifier: u32,
    min_dice_roll: u32,
    max_dice_roll: u32,
) -> f32 {
    let hit_chance: f32;

    // Can't let enemy ac be higher than hit mod cause unsigned integers don't like that
    if hit_modifier >= enemy_ac {
        hit_modifier = enemy_ac;
    }

    let mut needed_roll: u32 = enemy_ac - hit_modifier + 1;

    // Nat 1 always misses
    if needed_roll <= min_dice_roll {
        needed_roll = min_dice_roll;
    }

    // Crit always hits
    if needed_roll >= max_dice_roll {
        needed_roll = max_dice_roll - 1;
    }

    hit_chance = (max_dice_roll - needed_roll) as f32 / max_dice_roll as f32;

    return hit_chance;
}
