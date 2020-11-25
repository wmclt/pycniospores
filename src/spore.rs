use rayon::prelude::*;
use crate::{
    configuration::{
        FRICTION, MAX_FORCE_REACH, NUMBER_OF_SPORES, REPULSION_AMPLITUDE, UNIVERSE_HEIGHT,
        UNIVERSE_WIDTH,
    },
    vector::Vector,
};

pub struct Spores {
    pub positions: Vec<Vector>,
    pub speeds: Vec<Vector>,
    pub spore_types: Vec<u8>,
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
pub fn move_spores(spore_configs: &SporeConfigs, spores: &mut Spores) {
    let forces: Vec<Vector> = (0..NUMBER_OF_SPORES)
        .into_par_iter()
        .map(|index| calculate_forces(&spore_configs, index, &spores))
        .collect();

    let (new_poss, new_speeds) = (0..NUMBER_OF_SPORES as usize)
        .into_par_iter()
        .map(|index| (spores.positions[index], spores.speeds[index], forces[index]))
        .map(|(pos, speed, force)| {
            let new_speed = speed * FRICTION + force;
            (modulo_position(pos + new_speed), new_speed)
        })
        .unzip();

    spores.positions = new_poss;
    spores.speeds = new_speeds;
}

// parallellizing with crayon slows this function down! even with DOD
pub fn calculate_forces(spore_configs: &SporeConfigs, spore: u16, spores: &Spores) -> Vector {
    (0..NUMBER_OF_SPORES)
        .into_iter()
        .filter(|other| *other != spore)
        .map(|other| {
            (
                other,
                to_calibrated_dist(
                    spores.positions[other as usize],
                    spores.positions[spore as usize],
                ),
            )
        })
        .filter(|(other, dist)| {
            dist.tot_dist
                <= spore_configs.force_reaches[spores.spore_types[*other as usize] as usize]
        })
        .map(|(other, dist)| {
            calculate_force(spore_configs, spores.spore_types[other as usize], dist)
        }) // TODO not the problem
        .sum()
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
    if dist.tot_dist < repulsion_dist {
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
