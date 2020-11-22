use rayon::prelude::*;
use std::collections::HashMap;

use crate::vector::Vector;

pub const UNIVERSE_HEIGHT: f32 = 1600.0;
pub const UNIVERSE_WIDTH: f32 = 2560.0;

// TODO put in config file
pub const DEFAULT_REPULSION_DIST: f32 = 20.0; // Should be an absolute value.
pub const DEFAULT_REPULSION_AMPLITUDE: f32 = -5.0 * DEFAULT_FORCE_AMPLITUDE;
pub const DEFAULT_FORCE_AMPLITUDE: f32 = 0.12; //0.006
pub const DEFAULT_FORCE_REACH: f32 = 70.0; //70

pub const NUMBER_OF_SPORES: u16 = 3000;

const FRICTION: f32 = 0.94; // friction should be low!

pub type SporeConfigs = HashMap<SporeType, SporeConfig>;

pub struct Spore {
    pub id: u16,
    pub position: Vector,
    pub speed: Vector,
    pub spore_type: SporeType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SporeType {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Copy, Clone)]
pub struct SporeConfig {
    pub repulsion_dist: f32,
    pub force_factor: f32,
    pub force_reach: f32,
}

pub fn new_spore(
    id: u16,
    x_coord: f32,
    y_coord: f32,
    x_speed: f32,
    y_speed: f32,
    spore_type: SporeType,
) -> Spore {
    Spore {
        id,
        position: Vector {
            x: x_coord,
            y: y_coord,
        },
        speed: Vector {
            x: x_speed,
            y: y_speed,
        },
        spore_type,
    }
}

//  TWO loops
//  1. calculate forces
//  2. apply forces
//      3. update speeds: apply forces + friction to speeds
//      4. move according to speed
pub fn move_spores(spore_configs: &SporeConfigs, spores: &mut Vec<Spore>) {
    let forces: Vec<Vector> = spores
        .par_iter()
        .map(|spore| calculate_forces(&spore_configs, &spore, &spores))
        .collect();

    spores
        .par_iter_mut()
        .zip(forces.par_iter())
        .for_each(|(spore, force)| apply_and_move(spore, force));
}

pub struct Dist {
    vec: Vector,
    tot_dist: f32,
}

fn get_force_reach(spore_configs: &SporeConfigs, spore_type: &SporeType) -> f32 {
    spore_configs.get(&spore_type).unwrap().force_reach
}

// parallellizing with crayon slows this function down!
pub fn calculate_forces(
    spore_configs: &SporeConfigs,
    spore: &Spore,
    spores: &Vec<Spore>,
) -> Vector {
    spores
        .iter()
        .filter(|&other| spore.id != other.id)
        .map(|other| to_calibrated_dist(other, spore))
        .filter(|(other, dist)| dist.tot_dist <= get_force_reach(spore_configs, &other.spore_type))
        .map(|(other, dist)| calculate_force(spore_configs, &other, dist))
        .sum()
}

fn to_calibrated_dist<'a>(other: &'a Spore, spore: &Spore) -> (&'a Spore, Dist) {
    let uncalibrated_dist = other.position - spore.position;

    // recalibrate to account for wrap-around
    let x_dist = if uncalibrated_dist.x.abs() >= UNIVERSE_WIDTH - DEFAULT_FORCE_REACH {
        uncalibrated_dist.x - UNIVERSE_WIDTH * uncalibrated_dist.x.signum()
    } else {
        uncalibrated_dist.x
    };
    let y_dist = if uncalibrated_dist.y.abs() >= UNIVERSE_HEIGHT - DEFAULT_FORCE_REACH {
        uncalibrated_dist.y - UNIVERSE_HEIGHT * uncalibrated_dist.y.signum()
    } else {
        uncalibrated_dist.y
    };

    let tot_dist = (x_dist.powi(2) + y_dist.powi(2)).sqrt();
    (
        other,
        Dist {
            vec: Vector {
                x: x_dist,
                y: y_dist,
            },
            tot_dist,
        },
    )
}

pub fn calculate_force(spore_configs: &SporeConfigs, other: &Spore, dist: Dist) -> Vector {
    // if too close: acceleration away
    let repulsion_dist = get_repulsion_dist(spore_configs, &other.spore_type);
    if dist.tot_dist < repulsion_dist {
        let repulsion_force = (dist.tot_dist - repulsion_dist).powi(2)
            * DEFAULT_REPULSION_AMPLITUDE
            / repulsion_dist.powi(2);

        dist.vec * repulsion_force
    } else {
        scale_force(&spore_configs, &other.spore_type, dist)
    }
}

fn get_repulsion_dist(spore_configs: &SporeConfigs, spore_type: &SporeType) -> f32 {
    spore_configs.get(spore_type).unwrap().repulsion_dist
}

fn get_force_factor(spore_configs: &SporeConfigs, spore_type: &SporeType) -> f32 {
    spore_configs.get(&spore_type).unwrap().force_factor
}

// the force is linear to the distance |\./\ or |\.\/
fn scale_force(spore_configs: &SporeConfigs, spore_type: &SporeType, dist: Dist) -> Vector {
    let repulsion_dist = get_repulsion_dist(spore_configs, spore_type);
    let not_repulsion_dist = dist.tot_dist - repulsion_dist;
    let net_force_reach = get_force_reach(spore_configs, spore_type) - repulsion_dist;
    let dist_from_force_reach_center = (net_force_reach - not_repulsion_dist).abs();
    let scale = dist_from_force_reach_center / net_force_reach / 2.0;
    let factor = get_force_factor(spore_configs, spore_type);

    dist.vec * (factor * scale / dist.tot_dist)
}

fn apply_and_move(spore: &mut Spore, force: &Vector) {
    spore.speed = spore.speed * FRICTION + *force;
    let new_position = spore.position + spore.speed;
    spore.position = modulo_position(new_position);
}

fn modulo_position(position: Vector) -> Vector {
    Vector {
        x: (((position.x) % UNIVERSE_WIDTH) + UNIVERSE_WIDTH) % UNIVERSE_WIDTH,
        y: ((position.y % UNIVERSE_HEIGHT) + UNIVERSE_HEIGHT) % UNIVERSE_HEIGHT,
    }
}
