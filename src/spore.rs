use rand::{distributions::Standard, prelude::*};

use rayon::prelude::*;

const REPULSION_DISTANCE: f32 = 25.0; // Should be an absolute value.
const REPULSION_DISTANCE_SQRD: f32 = REPULSION_DISTANCE * REPULSION_DISTANCE;
const REPULSION_AMPLITUDE: f32 = -5.0 * DEFAULT_FORCE_AMPLITUDE;

const FORCE_REACH: f32 = 50.0; //70
const DEFAULT_FORCE_AMPLITUDE: f32 = 0.02; //0.006

// afgeleide gegevens
const NET_FORCE_REACH: f32 = FORCE_REACH - REPULSION_DISTANCE;
const HALF_NET_FORCE_REACH: f32 = NET_FORCE_REACH / 2.0;

const FRICTION: f32 = 0.98; // friction should be low!

pub const WINDOW_HEIGHT: f32 = 1080.0;//840.0; //1080.0;
pub const WINDOW_WIDTH: f32 = 1920.0;//1360.0; //1920.0;

const NUMBER_OF_SPORES: u16 = 500;

// const ZERO_FORCE: f32 = 0.0 * DEFAULT_FORCE_AMPLITUDE;
const ONE_THIRD_FORCE: f32 = 0.33 * DEFAULT_FORCE_AMPLITUDE;
const TWO_THIRDS_FORCE: f32 = 0.66 * DEFAULT_FORCE_AMPLITUDE;
const FULL_FORCE: f32 = 0.8 * DEFAULT_FORCE_AMPLITUDE;
const MINUS_HALF_FORCE: f32 = -0.5 * DEFAULT_FORCE_AMPLITUDE;
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
    Six,
}

pub fn generate_spores() -> Vec<Spore> {
    let mut results = Vec::new();
    let mut rng = rand::thread_rng();

    for id in 0..NUMBER_OF_SPORES {
        let x_coord: f32 = rng.gen_range(0.0, WINDOW_WIDTH);
        let y_coord: f32 = rng.gen_range(0.0, WINDOW_HEIGHT);
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
        match rng.gen_range(0, 6) {
            0 => SporeType::One,
            1 => SporeType::Two,
            2 => SporeType::Three,
            3 => SporeType::Four,
            4 => SporeType::Five,
            5 => SporeType::Six,
            _ => panic!("woups"),
        }
    }
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
        .map(|other| to_calibrated_dist(other, spore))
        .filter(|(_other, dist)| dist.x_dist <= FORCE_REACH && dist.y_dist <= FORCE_REACH)
        .filter(|(_other, dist)| dist.tot_dist <= FORCE_REACH)
        .map(|(other, dist)| calculate_force(other.spore_type, spore.spore_type, dist))
        .fold(zero_force(), |force, next_force| Force {
            x_force: force.x_force + next_force.x_force,
            y_force: force.y_force + next_force.y_force,
        })
}

fn to_calibrated_dist<'a>(other: &'a Spore, spore: &Spore) -> (&'a Spore, Dist) {
    let uncalibrated_x_dist = other.x_coord - spore.x_coord;
    let uncalibrated_y_dist = other.y_coord - spore.y_coord;

    // recalibrate to account for wraparound
    let x_dist = if uncalibrated_x_dist.abs() >= WINDOW_WIDTH - FORCE_REACH {
        uncalibrated_x_dist - WINDOW_WIDTH * uncalibrated_x_dist.signum()
    } else {
        uncalibrated_x_dist
    };
    let y_dist = if uncalibrated_y_dist.abs() >= WINDOW_HEIGHT - FORCE_REACH {
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

pub fn calculate_force(other: SporeType, _spore: SporeType, dist: Dist) -> Force {
    // if too close: acceleration away
    if dist.tot_dist < REPULSION_DISTANCE {
        let repulsion_force = (dist.tot_dist - REPULSION_DISTANCE).powi(2) * REPULSION_AMPLITUDE
            / REPULSION_DISTANCE
            / REPULSION_DISTANCE_SQRD;

        return Force {
            x_force: repulsion_force * dist.x_dist,
            y_force: repulsion_force * dist.y_dist,
        };
    }

    let force_factor: f32 = match other {
        SporeType::One => FULL_FORCE,
        SporeType::Two => ONE_THIRD_FORCE,
        SporeType::Three => MINUS_FULL_FORCE,
        SporeType::Four => ONE_THIRD_FORCE,
        SporeType::Five => TWO_THIRDS_FORCE,
        SporeType::Six => MINUS_HALF_FORCE,
    };
    scale_force(force_factor, dist)
}

// TODO try with different functions and try different distance functions -> like in video
// the force is linear to the distance |\./\ or |\.\/
fn scale_force(factor: f32, dist: Dist) -> Force {
    let not_repulsion_distance = dist.tot_dist - REPULSION_DISTANCE;
    let dist_from_force_reach_center = (NET_FORCE_REACH - not_repulsion_distance).abs();
    let scale = dist_from_force_reach_center / HALF_NET_FORCE_REACH;

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
