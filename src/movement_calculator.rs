use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    bucket::{get_neighbors, BucketCoord},
    configuration::{
        FRICTION, MAX_FORCE_REACH, REPULSION_AMPLITUDE, UNIVERSE_HEIGHT, UNIVERSE_WIDTH,
    },
    spore::{SporeConfigs, SporesState},
    vector::{Vector, ZERO_VECTOR},
};

pub fn calc_new_positions_and_speeds(
    spore_configs: &SporeConfigs,
    spores: &mut SporesState,
    (horz, vert): BucketCoord,
) -> (Vec<usize>, (Vec<Vector>, Vec<Vector>)) {
    let forces = calc_forces(spores, (horz, vert), spore_configs);
    update_spores_with_forces(spores, (horz, vert), forces)
}

fn calc_forces(
    spores: &mut SporesState,
    (horz, vert): BucketCoord,
    spore_configs: &SporeConfigs,
) -> Vec<Vector> {
    spores.positions[vert][horz]
        .par_iter()
        .map(|spore_position| {
            calculate_forces_on_spore(&spore_configs, *spore_position, spores, (horz, vert))
        })
        .collect()
}

// TODO (Vec<usize>, (Vec<Vector>, Vec<Vector>)) into NewSporesData
fn update_spores_with_forces(
    spores: &mut SporesState,
    (horz, vert): BucketCoord,
    forces: Vec<Vector>,
) -> (Vec<usize>, (Vec<Vector>, Vec<Vector>)) {
    (0..spores.positions[vert][horz].len())
        .into_par_iter()
        .map(|index| {
            (
                index,
                spores.positions[vert][horz][index],
                spores.speeds[vert][horz][index],
                forces[index],
            )
        })
        .map(|(index, pos, speed, force)| {
            let new_speed = speed * FRICTION + force;
            (index, (modulo_position(pos + new_speed), new_speed))
        })
        .unzip()
}

fn modulo_position(position: Vector) -> Vector {
    Vector {
        x: (((position.x) % UNIVERSE_WIDTH) + UNIVERSE_WIDTH) % UNIVERSE_WIDTH,
        y: ((position.y % UNIVERSE_HEIGHT) + UNIVERSE_HEIGHT) % UNIVERSE_HEIGHT,
    }
}

// parallellizing with crayon slows this function down! even with DOD
// TODO just pass neighbours?
pub fn calculate_forces_on_spore(
    spore_configs: &SporeConfigs,
    spore: Vector,
    spores: &SporesState,
    (horz, vert): BucketCoord,
) -> Vector {
    /*
     * 1. iter over neighbor (and self) bucket
     * 2. calculate total force from bucket
     * 3. sum forces of neighbors
     */
    get_neighbors(horz, vert)
        .iter()
        .map(|(neighb_horz, neighb_vert)| {
            let bucket_positions = &spores.positions[*neighb_vert][*neighb_horz];
            let bucket_spore_types = &spores.spore_types[*neighb_vert][*neighb_horz];
            calc_force_from_bucket(spore_configs, spore, bucket_positions, bucket_spore_types)
        })
        .sum()
}

fn calc_force_from_bucket(
    spore_configs: &SporeConfigs,
    spore: Vector,
    bucket_positions: &Vec<Vector>,
    bucket_spore_types: &Vec<u8>,
) -> Vector {
    /*
     * 1. iter over spores in bucket
     * 2. calc force of bucket spore on given spore
     * 3. filter out spores too far away
     * 4. sum forces
     */
    (0..bucket_positions.len())
        .into_iter()
        .map(|bucket_index| {
            (
                bucket_index,
                to_calibrated_dist(bucket_positions[bucket_index as usize], spore),
            )
        })
        .filter(|(bucket_index, dist)| {
            dist.scalar
                <= spore_configs.force_reaches[bucket_spore_types[*bucket_index as usize] as usize]
        })
        .map(|(bucket_index, dist)| {
            calculate_force(
                spore_configs,
                bucket_spore_types[bucket_index as usize],
                dist,
            )
        })
        .sum()
}

// the force function follows this function in terms of distance: either |\./\ or |\.\/
pub fn calculate_force(spore_configs: &SporeConfigs, spore_type: u8, dist: Dist) -> Vector {
    let repulsion_dist = spore_configs.repulsion_dists[spore_type as usize];
    if dist.scalar < 0.000001 {
        ZERO_VECTOR // because probably own spore (location)
    } else if dist.scalar < repulsion_dist {
        let repulsion_force =
            (dist.scalar - repulsion_dist).powi(2) * REPULSION_AMPLITUDE / repulsion_dist.powi(2);

        dist.vector * repulsion_force
    } else {
        scale_force(&spore_configs, spore_type as usize, dist)
    }
}

fn scale_force(spore_configs: &SporeConfigs, spore_type: usize, dist: Dist) -> Vector {
    let repulsion_dist = spore_configs.repulsion_dists[spore_type];
    let not_repulsion_dist = dist.scalar - repulsion_dist;
    let net_force_reach = spore_configs.force_reaches[spore_type] - repulsion_dist;
    let dist_from_force_reach_center = (net_force_reach - not_repulsion_dist).abs();
    let scale = dist_from_force_reach_center / net_force_reach / 2.0;
    let factor = spore_configs.force_factors[spore_type];

    dist.vector * (factor * scale / dist.scalar)
}

pub struct Dist {
    vector: Vector,
    scalar: f32,
}

fn to_calibrated_dist(other: Vector, spore: Vector) -> Dist {
    let uncalibrated_dist = other - spore;

    // recalibrate to account for wrap-around
    let x = if uncalibrated_dist.x.abs() >= UNIVERSE_WIDTH - MAX_FORCE_REACH {
        uncalibrated_dist.x - UNIVERSE_WIDTH * uncalibrated_dist.x.signum()
    } else {
        uncalibrated_dist.x
    };
    let y = if uncalibrated_dist.y.abs() >= UNIVERSE_HEIGHT - MAX_FORCE_REACH {
        uncalibrated_dist.y - UNIVERSE_HEIGHT * uncalibrated_dist.y.signum()
    } else {
        uncalibrated_dist.y
    };

    let tot_dist = (x.powi(2) + y.powi(2)).sqrt();

    Dist {
        vector: Vector { x, y },
        scalar: tot_dist,
    }
}
