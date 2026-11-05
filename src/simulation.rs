use rand::prelude::*;

fn roll_die(min_dice_roll: u32, max_dice_roll: u32) -> u32 {
    let roll_result: u32 = rand::rng().random_range(min_dice_roll..=max_dice_roll);

    return roll_result;
}
