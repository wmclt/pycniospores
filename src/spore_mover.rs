use std::usize;

use crate::{
    bucket::get_bucket_from_pos,
    configuration::{NBR_HORZ_BUCKETS, NBR_VERT_BUCKETS},
    movement_calculator::calc_new_positions_and_speeds,
    spore::{SporeConfigs, SporesState},
    vector::Vector,
};

//  TWO loops
//  1. calculate forces
//  2. apply forces
//      3. update speeds: apply forces + friction to speeds
//      4. move according to speed
pub fn move_spores(spore_configs: &SporeConfigs, spores_state: &mut SporesState) {
    for vert in 0..NBR_VERT_BUCKETS {
        for horz in 0..NBR_HORZ_BUCKETS {
            move_spores_in_bucket(spore_configs, spores_state, (horz as usize, vert as usize));
        }
    }

    for vert in 0..NBR_VERT_BUCKETS {
        for horz in 0..NBR_HORZ_BUCKETS {
            let bucket_movements = calc_bucket_movements(&spores_state, horz, vert);
            copy_spores_to_new_bucket(&bucket_movements, spores_state);
            remove_spores_from_old_buckets(&bucket_movements, spores_state, vert, horz);
        }
    }
}

fn move_spores_in_bucket(
    spore_configs: &SporeConfigs,
    spores: &mut SporesState,
    (horz, vert): (usize, usize),
) {
    // new positions & speeds
    let (indexes, (new_poss, new_speeds)) =
        calc_new_positions_and_speeds(spore_configs, spores, (horz, vert));

    for spore in indexes {
        spores.positions[vert][horz][spore] = new_poss[spore];
        spores.speeds[vert][horz][spore] = new_speeds[spore];
    }
}

fn copy_spores_to_new_bucket(bucket_movements: &Vec<SporeBucketMovement>, spores: &mut SporesState) {
    for movement in bucket_movements {
        let (new_horz, new_vert) = movement.new_bucket_coord;
        let (pos, speed, spore_type) = movement.spore_data;

        spores.positions[new_vert][new_horz].push(pos);
        spores.speeds[new_vert][new_horz].push(speed);
        spores.spore_types[new_vert][new_horz].push(spore_type);
    }
}

/// remove spores that have moved (=only keep spores that haven't moved) (uses black magic :/)
fn remove_spores_from_old_buckets(bucket_movements: &Vec<SporeBucketMovement>, spores: &mut SporesState, vert: usize, horz: usize) {
    let bucket_movement: Vec<usize> = bucket_movements
        .iter()
        .map(|movement| movement.index_in_old_bucket)
        .collect();
    let mut i: usize = 0;
    spores.positions[vert][horz].retain(|_| (!bucket_movement.contains(&i), i += 1).0);
    i = 0;
    spores.speeds[vert][horz].retain(|_| (!bucket_movement.contains(&i), i += 1).0);
    i = 0;
    spores.spore_types[vert][horz].retain(|_| (!bucket_movement.contains(&i), i += 1).0);
}


struct SporeBucketMovement {
    index_in_old_bucket: usize,
    new_bucket_coord: (usize, usize),
    spore_data: (Vector, Vector, u8),
}

fn calc_bucket_movements(
    spores: &SporesState,
    horz: usize,
    vert: usize,
) -> Vec<SporeBucketMovement> {
    let mut bucket_movements = Vec::with_capacity(spores.positions[vert][horz].len());

    for old_bucket_index in 0..spores.positions[vert][horz].len() {
        let new_bucket_coord = get_bucket_from_pos(spores.positions[vert][horz][old_bucket_index]);
        if new_bucket_coord != (horz, vert) {
            bucket_movements.push(SporeBucketMovement {
                index_in_old_bucket: old_bucket_index,
                new_bucket_coord,
                spore_data: (
                    spores.positions[vert][horz][old_bucket_index],
                    spores.speeds[vert][horz][old_bucket_index],
                    spores.spore_types[vert][horz][old_bucket_index],
                ),
            });
        }
    }
    bucket_movements
}