use crate::{
    bucket::get_bucket_from_vec,
    bucket::get_neighbors,
    configuration::{
        FRICTION, MAX_FORCE_REACH, NBR_HORZ_BUCKETS, NBR_VERT_BUCKETS, REPULSION_AMPLITUDE,
        UNIVERSE_HEIGHT, UNIVERSE_WIDTH,
    },
    vector::{Vector, ZERO_VECTOR},
};
use rayon::prelude::*;

pub struct SporesMatrix {
    pub positions: Vec<Vec<Vec<Vector>>>,
    pub speeds: Vec<Vec<Vec<Vector>>>,
    pub spore_types: Vec<Vec<Vec<u8>>>,
}

#[derive(Debug)]
pub struct SporeConfigs {
    pub repulsion_dists: Vec<f32>,
    pub force_factors: Vec<f32>,
    pub force_reaches: Vec<f32>,
}

//  TWO loops
//  1. calculate forces
//  2. apply forces
//      3. update speeds: apply forces + friction to speeds
//      4. move according to speed
pub fn move_spores(spore_configs: &SporeConfigs, spores: &mut SporesMatrix) {
    for vert in 0..NBR_VERT_BUCKETS {
        for horz in 0..NBR_HORZ_BUCKETS {
            move_spores_in_bucket(spore_configs, spores, (horz as usize, vert as usize));
        }
    }

    for vert in 0..NBR_VERT_BUCKETS {
        for horz in 0..NBR_HORZ_BUCKETS {
            let mut to_move = Vec::with_capacity(spores.positions[vert][horz].len());

            for index in 0..spores.positions[vert][horz].len() {
                let (new_horz, new_vert) = get_bucket_from_vec(spores.positions[vert][horz][index]);
                if (new_horz, new_vert) != (horz, vert) {
                    to_move.push((
                        index,
                        (new_horz, new_vert),
                        (
                            spores.positions[vert][horz][index],
                            spores.speeds[vert][horz][index],
                            spores.spore_types[vert][horz][index],
                        ),
                    ));
                }
            }

            // copy spores
            for &(_, (new_horz, new_vert), (pos, speed, spore_type)) in &to_move {
                spores.positions[new_vert][new_horz].push(pos);
                spores.speeds[new_vert][new_horz].push(speed);
                spores.spore_types[new_vert][new_horz].push(spore_type);
            }

            // remove spores
            let spores_to_move: Vec<&usize> = to_move.iter().map(|(i, _, _)| i).collect();
            let mut i: usize = 0;
            spores.positions[vert][horz].retain(|_| (!spores_to_move.contains(&&i), i += 1).0);
            i = 0;
            spores.speeds[vert][horz].retain(|_| (!spores_to_move.contains(&&i), i += 1).0);
            i = 0;
            spores.spore_types[vert][horz].retain(|_| (!spores_to_move.contains(&&i), i += 1).0);
        }
    }
}

fn move_spores_in_bucket(
    spore_configs: &SporeConfigs,
    spores: &mut SporesMatrix,
    (horz, vert): (usize, usize),
) {
    let forces: Vec<Vector> = spores.positions[vert][horz]
        .par_iter()
        .map(|spore_position| {
            calculate_forces(&spore_configs, *spore_position, &spores, (horz, vert))
        })
        .collect();

    let (indexes, (new_poss, new_speeds)): (Vec<usize>, (Vec<Vector>, Vec<Vector>)) =
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
            .unzip();

    for spore in indexes {
        spores.positions[vert][horz][spore] = new_poss[spore];
        spores.speeds[vert][horz][spore] = new_speeds[spore];
    }
}

// parallellizing with crayon slows this function down! even with DOD
// TODO just pass neighbours?
pub fn calculate_forces(
    spore_configs: &SporeConfigs,
    spore: Vector,
    spores: &SporesMatrix,
    (horz, vert): (usize, usize),
) -> Vector {
    /*
     * 1. iter over spores in bucket
     * 2. iter over neighbor (and self) bucket
     * 3. iter over spores of neighbors (and self)
     * 4. calc force
     */
    let mut forces = vec![];
    for (neighb_horz, neighb_vert) in get_neighbors(horz, vert).iter() {
        let positions = &spores.positions[*neighb_vert][*neighb_horz];
        let spore_types = &spores.spore_types[*neighb_vert][*neighb_horz];
        let bucket_force = (0..positions.len())
            .into_iter()
            .map(|bucket_index| {
                (
                    bucket_index,
                    to_calibrated_dist(positions[bucket_index as usize], spore),
                )
            })
            .filter(|(bucket_index, dist)| {
                dist.tot_dist
                    <= spore_configs.force_reaches[spore_types[*bucket_index as usize] as usize]
            })
            .map(|(bucket_index, dist)| {
                calculate_force(spore_configs, spore_types[bucket_index as usize], dist)
            })
            .sum();
        forces.push(bucket_force);
    }

    forces.into_iter().sum()
}

pub struct Dist {
    vec: Vector,
    tot_dist: f32,
}

fn to_calibrated_dist<'a>(other: Vector, spore: Vector) -> Dist {
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
        vec: Vector { x, y },
        tot_dist,
    }
}

pub fn calculate_force(spore_configs: &SporeConfigs, spore_type: u8, dist: Dist) -> Vector {
    // if too close: acceleration away
    // let repulsion_dist = get_repulsion_dist(spore_configs, other);
    let repulsion_dist = spore_configs.repulsion_dists[spore_type as usize];
    if dist.tot_dist < 0.0001 {
        ZERO_VECTOR
    } else if dist.tot_dist < repulsion_dist {
        let repulsion_force =
            (dist.tot_dist - repulsion_dist).powi(2) * REPULSION_AMPLITUDE / repulsion_dist.powi(2);

        dist.vec * repulsion_force
    } else {
        scale_force(&spore_configs, spore_type as usize, dist)
    }
}

// the force is linear to the distance |\./\ or |\.\/
fn scale_force(spore_configs: &SporeConfigs, spore_type: usize, dist: Dist) -> Vector {
    let repulsion_dist = spore_configs.repulsion_dists[spore_type];
    let not_repulsion_dist = dist.tot_dist - repulsion_dist;
    let net_force_reach = spore_configs.force_reaches[spore_type] - repulsion_dist;
    let dist_from_force_reach_center = (net_force_reach - not_repulsion_dist).abs();
    let scale = dist_from_force_reach_center / net_force_reach / 2.0;
    let factor = spore_configs.force_factors[spore_type];

    dist.vec * (factor * scale / dist.tot_dist)
}

fn modulo_position(position: Vector) -> Vector {
    Vector {
        x: (((position.x) % UNIVERSE_WIDTH) + UNIVERSE_WIDTH) % UNIVERSE_WIDTH,
        y: ((position.y % UNIVERSE_HEIGHT) + UNIVERSE_HEIGHT) % UNIVERSE_HEIGHT,
    }
}
