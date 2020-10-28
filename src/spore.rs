use rayon::prelude::*;
use std::collections::HashMap;

pub const WINDOW_HEIGHT: f32 = 800.0; //1200.0;
pub const WINDOW_WIDTH: f32 = 1280.0; //1920.0

// TODO put in config file
pub const DEFAULT_REPULSION_DIST: f32 = 20.0; // Should be an absolute value.
pub const DEFAULT_REPULSION_AMPLITUDE: f32 = -5.0 * DEFAULT_FORCE_AMPLITUDE;
pub const DEFAULT_FORCE_AMPLITUDE: f32 = 0.12; //0.006
pub const DEFAULT_FORCE_REACH: f32 = 70.0; //70

pub const NUMBER_OF_SPORES: u16 = 300;

const FRICTION: f32 = 0.94; // friction should be low!

pub type SporeConfigs = HashMap<SporeType, SporeConfig>;

pub struct Spore {
    pub id: u16,
    pub x_coord: f32,
    pub y_coord: f32,
    x_speed: f32,
    y_speed: f32,
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
        x_coord,
        y_coord,
        x_speed,
        y_speed,
        spore_type,
    }
}

//  TWO loops
//  1. calculate
//      b. find neighbours close enough
//          * square filter
//          * round filter
//      c. calculate forces
// 2. apply
//      3. update speeds: apply forces + friction to speeds
//      4. move according to speed
// TODO use spore_configs
pub fn move_spores(spore_configs: &SporeConfigs, spores: &mut Vec<Spore>) {
    let forces: Vec<Force> = spores
        .par_iter()
        .map(|spore| calculate_forces(&spore_configs, &spore, &spores))
        .collect();

    spores
        .par_iter_mut()
        .zip(forces.par_iter())
        .for_each(|(spore, force)| apply_force_and_move(spore, force));
}

pub struct Dist {
    x_dist: f32,
    y_dist: f32,
    tot_dist: f32,
}

pub struct Force {
    x_force: f32,
    y_force: f32,
}

fn get_force_reach(spore_configs: &SporeConfigs, spore_type: &SporeType) -> f32 {
    spore_configs.get(&spore_type).unwrap().force_reach
}

pub fn calculate_forces(spore_configs: &SporeConfigs, spore: &Spore, spores: &Vec<Spore>) -> Force {
    spores
        .iter()
        .filter(|&other| spore.id != other.id)
        .map(|other| to_calibrated_dist(other, spore))
        .filter(|(other, dist)| {
            dist.x_dist <= get_force_reach(spore_configs, &other.spore_type)
                && dist.y_dist <= get_force_reach(spore_configs, &other.spore_type)
        })
        .filter(|(other, dist)| dist.tot_dist <= get_force_reach(spore_configs, &other.spore_type))
        .map(|(other, dist)| calculate_force(spore_configs, &other, dist))
        .fold(zero_force(), |force, next_force| Force {
            x_force: force.x_force + next_force.x_force,
            y_force: force.y_force + next_force.y_force,
        })
}

fn to_calibrated_dist<'a>(other: &'a Spore, spore: &Spore) -> (&'a Spore, Dist) {
    let uncalibrated_x_dist = other.x_coord - spore.x_coord;
    let uncalibrated_y_dist = other.y_coord - spore.y_coord;

    // recalibrate to account for wrap-around
    let x_dist = if uncalibrated_x_dist.abs() >= WINDOW_WIDTH - DEFAULT_FORCE_REACH {
        uncalibrated_x_dist - WINDOW_WIDTH * uncalibrated_x_dist.signum()
    } else {
        uncalibrated_x_dist
    };
    let y_dist = if uncalibrated_y_dist.abs() >= WINDOW_HEIGHT - DEFAULT_FORCE_REACH {
        uncalibrated_y_dist - WINDOW_HEIGHT * uncalibrated_y_dist.signum()
    } else {
        uncalibrated_y_dist
    };

    let tot_dist = (x_dist.powi(2) + y_dist.powi(2)).sqrt();
    (
        other,
        Dist {
            x_dist,
            y_dist,
            tot_dist,
        },
    )
}

fn zero_force() -> Force {
    Force {
        x_force: 0.0,
        y_force: 0.0,
    }
}

pub fn calculate_force(spore_configs: &SporeConfigs, other: &Spore, dist: Dist) -> Force {
    // if too close: acceleration away
    let repulsion_dist = get_repulsion_dist(spore_configs, &other.spore_type);
    if dist.tot_dist < repulsion_dist {
        let repulsion_force = (dist.tot_dist - repulsion_dist).powi(2)
            * DEFAULT_REPULSION_AMPLITUDE
            / repulsion_dist.powi(2);

        return Force {
            x_force: repulsion_force * dist.x_dist,
            y_force: repulsion_force * dist.y_dist,
        };
    }

    scale_force(&spore_configs, &other.spore_type, dist)
}

fn get_repulsion_dist(spore_configs: &SporeConfigs, spore_type: &SporeType) -> f32 {
    spore_configs.get(spore_type).unwrap().repulsion_dist
}

fn get_force_factor(spore_configs: &SporeConfigs, spore_type: &SporeType) -> f32 {
    spore_configs.get(&spore_type).unwrap().force_factor
}

// the force is linear to the distance |\./\ or |\.\/
fn scale_force(spore_configs: &SporeConfigs, spore_type: &SporeType, dist: Dist) -> Force {
    let repulsion_dist = get_repulsion_dist(spore_configs, spore_type);
    let not_repulsion_dist = dist.tot_dist - repulsion_dist;
    let net_force_reach = get_force_reach(spore_configs, spore_type) - repulsion_dist;
    let dist_from_force_reach_center = (net_force_reach - not_repulsion_dist).abs();
    let scale = dist_from_force_reach_center / net_force_reach / 2.0;
    let factor = get_force_factor(spore_configs, spore_type);

    Force {
        x_force: dist.x_dist * factor / dist.tot_dist * scale,
        y_force: dist.y_dist * factor / dist.tot_dist * scale,
    }
}

fn apply_force_and_move(spore: &mut Spore, force: &Force) {
    spore.x_speed = spore.x_speed * FRICTION + force.x_force;
    spore.y_speed = spore.y_speed * FRICTION + force.y_force;

    let new_x_coord = spore.x_coord + spore.x_speed;
    let new_y_coord = spore.y_coord + spore.y_speed;

    spore.x_coord = (((new_x_coord) % WINDOW_WIDTH) + WINDOW_WIDTH) % WINDOW_WIDTH;
    spore.y_coord = ((new_y_coord % WINDOW_HEIGHT) + WINDOW_HEIGHT) % WINDOW_HEIGHT;
}
