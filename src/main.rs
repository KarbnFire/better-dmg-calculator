use std::f32;

const MIN_DICE_ROLL: u32 = 1;
const MAX_D4_ROLL: u32 = 4;
const MAX_D6_ROLL: u32 = 6;
const MAX_D8_ROLL: u32 = 8;
const MAX_D10_ROLL: u32 = 10;
const MAX_D12_ROLL: u32 = 12;
const MAX_D20_ROLL: u32 = 20;
const MAX_D100_ROLL: u32 = 100;
const ENABLE_DEBUG: bool = true;
const NATURAL_EXCEPTIONS: bool = false;

fn main() {
    //println!("Roll result is {}", roll_die());
    let enemy_ac: u32 = 2;
    let hit_mod: u32 = 0;

    let single_die_roll = calculate_single_dice_hit_chance(
        enemy_ac,
        hit_mod,
        MIN_DICE_ROLL,
        MAX_D20_ROLL,
        NATURAL_EXCEPTIONS,
    );
    let advantage_layers: u32 = 2;
    let disadvantage_layers: u32 = 0;

    let check_chance = calculate_check(single_die_roll, advantage_layers, disadvantage_layers);

    //println!(
    //"Hit chance on {enemy_ac} AC {hit_mod} hit mod is {}%",
    //(calculate_single_dice_hit_chance(enemy_ac, hit_mod, MIN_DICE_ROLL, MAX_D20_ROLL) * 100.0)
    //as u32
    //);

    if ENABLE_DEBUG {
        println!(
            "[DEBUG] Check
- Single die chance {single_die_roll}
- advantage layers {advantage_layers}
- disadvantage layers {disadvantage_layers}
-------------------------------------------------
Chance is {}%",
            ((check_chance * 1000000.0).floor() / 10000.0) // 2 decimal places
        );
    }
}

fn calculate_single_dice_hit_chance(
    mut enemy_ac: u32,
    mut hit_modifier: u32,
    min_dice_roll: u32,
    max_dice_roll: u32,
    natural_exceptions: bool,
) -> f32 {
    let hit_chance: f32;
    let mut needed_roll: u32;

    // Can't let enemy ac be higher than hit mod cause unsigned integers don't like that
    // -1 cause meets it beats it
    if hit_modifier >= enemy_ac - 1 {
        hit_modifier = 0;
        enemy_ac = 0;
    }

    // Can always miss with a Nat 1 with natural exceptions enabled
    if enemy_ac <= min_dice_roll {
        if natural_exceptions {
            if ENABLE_DEBUG {
                println!("[DEBUG] Nat 1 needed to fail");
            }
            needed_roll = min_dice_roll;
        } else {
            if ENABLE_DEBUG {
                println!(
                    "[DEBUG] Can't fail because hit modifier is >= enemy AC and natural exceptions are disabled"
                );
            }
            needed_roll = 0;
        }
    } else {
        // -1 cause meets it beats it
        needed_roll = enemy_ac - hit_modifier - 1;
    }

    // Crit always hits with natural exceptions enabled
    if needed_roll >= max_dice_roll {
        if natural_exceptions {
            if ENABLE_DEBUG {
                println!("[DEBUG] Nat 20 needed to succeed");
            }
            needed_roll = max_dice_roll - 1;
        } else {
            if ENABLE_DEBUG {
                println!(
                    "[DEBUG] Can't succeed because the Needed roll exceeds the maximum roll value of the dice and natural exceptions are disabled"
                );
            }
        }
    }

    hit_chance = (max_dice_roll - needed_roll) as f32 / max_dice_roll as f32;

    return hit_chance;
}

fn calculate_check(
    single_die_chance: f32,
    mut advantage_layers: u32,
    mut disadvantage_layers: u32,
) -> f32 {
    let calculated_chance: f32;

    // Advantage and disadvantage cancel each other out
    if advantage_layers == disadvantage_layers {
        return single_die_chance;
    }

    if advantage_layers > disadvantage_layers {
        advantage_layers -= disadvantage_layers;
        disadvantage_layers = 0;
    }

    if disadvantage_layers > advantage_layers {
        disadvantage_layers -= advantage_layers;
        advantage_layers = 0;
    }

    if advantage_layers > 0 && disadvantage_layers > 0 {
        // TODO - Throw exception or something here
        println!(
            "Something went wrong! advantage is {} and disadvantage is {}. They should have cancelled each other out.",
            advantage_layers, disadvantage_layers
        );
    }

    if ENABLE_DEBUG {
        println!(
            "[DEBUG] Effective advantage is {advantage_layers}, effective disadvantage is {disadvantage_layers}"
        );
    }

    // Advantage Ex - Log
    //  x - Chance with advantage
    //  y - Chance with single die
    //  z - Advantage layers
    //  x = 1 - (1 - y) ^ (z + 1)
    //  0.998 = 1 - (1 - 0.95) ^ (1 + 1)
    // Disadvantage Ex - Power/exponent
    //  x - Chance with disadvantage
    //  y - Chance with single die
    //  z - Disadvantage layers
    //  x = y ^ (z + 1)
    //  0.25 = 0.5 ^ (1 + 1)
    if advantage_layers > 0 {
        calculated_chance = 1.0 - (1.0 - single_die_chance).powf((advantage_layers + 1) as f32);
    } else if disadvantage_layers > 0 {
        calculated_chance = single_die_chance.powf((disadvantage_layers + 1) as f32);
    } else {
        // TODO - Throw exception or something here
        calculated_chance = 0.0;
        println!(
            "Something went wrong! advantage is {} and disadvantage is {}. One of them should be > 0.",
            advantage_layers, disadvantage_layers
        );
    }

    return calculated_chance;
}
