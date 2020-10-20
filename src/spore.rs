use rand::{distributions::Standard, prelude::*};

use rayon::prelude::*;

const REPULSION_DISTANCE: f32 = 10.0; // Should be an absolute value.
const REPULSION_AMPLITUDE: f32 = -0.05 * DEFAULT_FORCE_AMPLITUDE;

const FORCE_REACH: f32 = 70.0;
const DEFAULT_FORCE_AMPLITUDE: f32 = 0.005;

const FRICTION: f32 = 0.9875; // friction should be low!

pub const WINDOW_HEIGHT: f32 = 800.0;
pub const WINDOW_WIDTH: f32 = 1200.0;

const NUMBER_OF_SPORES: u16 = 200;

const ZERO_FORCE: f32 = 0.0 * DEFAULT_FORCE_AMPLITUDE;
const ONE_THIRD_FORCE: f32 = 0.33 * DEFAULT_FORCE_AMPLITUDE;
const TWO_THIRDS_FORCE: f32 = 0.66 * DEFAULT_FORCE_AMPLITUDE;
const FULL_FORCE: f32 = 1.0 * DEFAULT_FORCE_AMPLITUDE;
const MINUS_HALF_FORCE: f32 = 0.5 * DEFAULT_FORCE_AMPLITUDE;
const MINUS_FULL_FORCE: f32 = -0.9 * DEFAULT_FORCE_AMPLITUDE;

pub struct Spore {
    pub id: u16,
    pub x_coord: f32,
    pub y_coord: f32,
    x_speed: f32,
    y_speed: f32,
    pub spore_type: SporeType,
}

#[derive(Debug, Copy, Clone)]
pub enum SporeType {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub fn generate_spores(max_x: f32, max_y: f32) -> Vec<Spore> {
    let mut results = Vec::new();
    let mut rng = rand::thread_rng();

    for id in 0..NUMBER_OF_SPORES {
        let x_coord: f32 = rng.gen_range(0.0, max_x);
        let y_coord: f32 = rng.gen_range(0.0, max_y);
        let x_speed: f32 = 0.0;
        let y_speed: f32 = 0.0;

        results.push(new_spore(
            id,
            x_coord,
            y_coord,
            x_speed,
            y_speed,
            rand::random(),
        ));
    }
    results
}

impl Distribution<SporeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SporeType {
        match rng.gen_range(0, 4) {
            0 => SporeType::One,
            1 => SporeType::Two,
            2 => SporeType::Three,
            3 => SporeType::Four,
            _ => SporeType::Five,
        }
    }
}

// TODO spores will start with speed 0
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
pub fn move_spores(spores: &mut Vec<Spore>) {
    let forces: Vec<Force> = spores
        .par_iter()
        .map(|spore| calculate_forces(&spore, &spores))
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

pub fn calculate_forces(spore: &Spore, spores: &Vec<Spore>) -> Force {
    spores
        .iter()
        .filter(|&other| spore.id != other.id)
        .filter(|&other| {
            (other.x_coord - spore.x_coord).abs() <= FORCE_REACH
                && (other.y_coord - spore.y_coord).abs() <= FORCE_REACH
        })
        .map(|other| {
            let x_dist = other.x_coord - spore.x_coord;
            let y_dist = other.y_coord - spore.y_coord;
            let tot_dist = (x_dist.powi(2) + y_dist.powi(2)).sqrt();
            (
                other,
                Dist {
                    x_dist,
                    y_dist,
                    tot_dist,
                },
            )
        })
        .filter(|(_other, dist)| dist.tot_dist <= FORCE_REACH)
        .map(|(other, dist)| calculate_force(other.spore_type, spore.spore_type, dist))
        // .map(|(other, dist)| scale_force(5.0, 1.0, 1.0))
        .fold(zero_force(), |force, next_force| Force {
            x_force: force.x_force + next_force.x_force,
            y_force: force.y_force + next_force.y_force,
        })
}

fn zero_force() -> Force {
    Force {
        x_force: 0.0,
        y_force: 0.0,
    }
}

// from <right> on <down>	    One	Two	Three	Four	Five
//                      One	    0	0.66	1	0	-0.5
//                      Two	    0.66	-1	-0.5	0	-0.5
//                      Three	1	0.33	1	0.33	0.66
//                      Four	-0.5	1	0.33	0.66	0.33
//                      Five	0.66	0.33	-0.5	0.66	1
pub fn calculate_force(other: SporeType, spore: SporeType, dist: Dist) -> Force {
    // if too close: acceleration away
    if dist.tot_dist < REPULSION_DISTANCE {
        return Force {
            x_force: 1.0 / ((dist.x_dist).abs() - REPULSION_DISTANCE / 2.0).exp2()
                * REPULSION_AMPLITUDE
                * dist.x_dist.signum()
                + 0.5 * dist.x_dist.signum(),
            y_force: 1.0 / ((dist.y_dist).abs() - REPULSION_DISTANCE / 2.0).exp2()
                * REPULSION_AMPLITUDE
                * dist.y_dist.signum()
                + 0.5 * dist.y_dist.signum(),
        };
    }

    let force_factor: f32 = match spore {
        SporeType::One => match other {
            SporeType::One => ZERO_FORCE,
            SporeType::Two => MINUS_HALF_FORCE,
            SporeType::Three => FULL_FORCE,
            SporeType::Four => ZERO_FORCE,
            SporeType::Five => MINUS_HALF_FORCE,
        },
        SporeType::Two => match other {
            SporeType::One => TWO_THIRDS_FORCE,
            SporeType::Two => MINUS_FULL_FORCE,
            SporeType::Three => MINUS_HALF_FORCE,
            SporeType::Four => ZERO_FORCE,
            SporeType::Five => MINUS_HALF_FORCE,
        },
        SporeType::Three => match other {
            SporeType::One => FULL_FORCE,
            SporeType::Two => ONE_THIRD_FORCE,
            SporeType::Three => MINUS_FULL_FORCE,
            SporeType::Four => ONE_THIRD_FORCE,
            SporeType::Five => TWO_THIRDS_FORCE,
        },
        SporeType::Four => match other {
            SporeType::One => MINUS_HALF_FORCE,
            SporeType::Two => FULL_FORCE,
            SporeType::Three => ONE_THIRD_FORCE,
            SporeType::Four => TWO_THIRDS_FORCE,
            SporeType::Five => ONE_THIRD_FORCE,
        },
        SporeType::Five => match other {
            SporeType::One => TWO_THIRDS_FORCE,
            SporeType::Two => ONE_THIRD_FORCE,
            SporeType::Three => MINUS_HALF_FORCE,
            SporeType::Four => TWO_THIRDS_FORCE,
            SporeType::Five => FULL_FORCE,
        },
    };
    scale_force(force_factor, dist.x_dist, dist.y_dist)
}

// TODO the force is linear to the distance
// TODO maybe try with different functions
// TODO also try different distance functions?
fn scale_force(factor: f32, x_dist: f32, y_dist: f32) -> Force {
    Force {
        x_force: x_dist * factor,
        y_force: y_dist * factor,
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
