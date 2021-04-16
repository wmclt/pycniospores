use crate::{bucket::get_bucket, configuration::{EXP_NBR_SPORES_PER_BUCKET, GENERATE_RANDOM_START_POSITIONS, MAX_FORCE_AMPLITUDE, MAX_FORCE_REACH, MAX_REPULSION_DIST, NBR_HORZ_BUCKETS, NBR_VERT_BUCKETS, NUMBER_OF_CONFIGS, NUMBER_OF_SPORES, UNIVERSE_HEIGHT, UNIVERSE_WIDTH}, spore::{SporeConfigs, SporesState}, vector::{Vector, ZERO_VECTOR}};
use rand::prelude::*;

pub fn generate_spore_configs() -> SporeConfigs {
    let mut rng = rand::thread_rng();

    // TODO make arrays -> because so small. Arrays are prob loaded to the CPU cache, while Vector has one more level of redirection.
    let mut repulsion_dists = Vec::with_capacity(6);
    let mut force_factors = Vec::with_capacity(6);
    let mut force_reaches = Vec::with_capacity(6);

    if GENERATE_RANDOM_START_POSITIONS {
        (0..NUMBER_OF_CONFIGS).for_each(|_| {
            repulsion_dists.push(rng.gen_range(0.08, 1.2) * MAX_REPULSION_DIST);
            force_factors.push(
                rng.gen_range(0.15, 1.0)
                    * if rng.gen_bool(0.65) { 1.0 } else { -1.0 }
                    * MAX_FORCE_AMPLITUDE,
            );
            force_reaches.push(rng.gen_range(0.20, 1.0) * MAX_FORCE_REACH);
        });

        SporeConfigs {
            repulsion_dists,
            force_factors,
            force_reaches,
        }
    } else {
        get_previous_configs()
    }
}

pub fn generate_spores() -> SporesState {
    let mut rng = rand::thread_rng();

    let mut positions = vec![
        vec![Vec::with_capacity(EXP_NBR_SPORES_PER_BUCKET); NBR_HORZ_BUCKETS];
        NBR_VERT_BUCKETS
    ];
    let mut speeds = vec![
        vec![Vec::with_capacity(EXP_NBR_SPORES_PER_BUCKET); NBR_HORZ_BUCKETS];
        NBR_VERT_BUCKETS
    ];
    let mut spore_types = vec![
        vec![Vec::with_capacity(EXP_NBR_SPORES_PER_BUCKET); NBR_HORZ_BUCKETS];
        NBR_VERT_BUCKETS
    ];

    for _ in 0..NUMBER_OF_SPORES {
        let x: f32 = rng.gen_range(0.0, UNIVERSE_WIDTH);
        let y: f32 = rng.gen_range(0.0, UNIVERSE_HEIGHT);

        let (horz, vert) = get_bucket(x, y);

        // println!("{}, {}", horz, vert);
        positions[vert][horz].push(Vector { x, y });
        speeds[vert][horz].push(ZERO_VECTOR);
        spore_types[vert][horz].push(rng.gen_range(0, NUMBER_OF_CONFIGS));
    }
    SporesState {
        positions,
        speeds,
        spore_types,
    }
}

// pub positions: [[Vec<Vector>; NBR_HORZ_BUCKETS]; NBR_VERT_BUCKETS],
// pub speeds: [[Vec<Vector>; NBR_HORZ_BUCKETS]; NBR_VERT_BUCKETS],
// pub spore_types: [[Vec<u8>; NBR_HORZ_BUCKETS]; NBR_VERT_BUCKETS],

// replace with previous spore configuration
fn get_previous_configs() -> SporeConfigs {
    SporeConfigs {
        repulsion_dists: vec![9.82, 5.97, 9.16, 17.59, 17.57, 5.08],
        force_factors: vec![0.08, -0.04, 0.06, 0.07, 0.10, -0.08],
        force_reaches: vec![27.41, 58.38, 38.75, 36.55, 52.39, 65.82],
    }
}
