use crate::{
    bucket::get_bucket,
    configuration::{
        MAX_FORCE_AMPLITUDE, MAX_FORCE_REACH, MAX_REPULSION_DIST, NR_BUCKETS, NR_HORZ_BUCKETS,
        NR_VERT_BUCKETS, NUMBER_OF_CONFIGS, UNIVERSE_HEIGHT, UNIVERSE_WIDTH,
        USE_PREVIOUS_CONFIGURATIONS,
    },
    spore::{SporeConfigs, SporesState},
    vector::{Vector, ZERO_VECTOR},
};
use rand::prelude::*;

pub fn generate_spore_configs() -> SporeConfigs {
    if USE_PREVIOUS_CONFIGURATIONS {
        return PREVIOUS_CONFIGS;
    }

    let mut rng = rand::rng();

    let mut repulsion_dists = [0.0; NUMBER_OF_CONFIGS];
    let mut force_factors = [0.0; NUMBER_OF_CONFIGS];
    let mut force_reaches = [0.0; NUMBER_OF_CONFIGS];

    (0..NUMBER_OF_CONFIGS).for_each(|index| {
        repulsion_dists[index] = rng.random_range(0.08..=1.2) * MAX_REPULSION_DIST;
        force_factors[index] = rng.random_range(0.15..=1.0)
            * if rng.random_bool(0.65) { 1.0 } else { -1.0 }
            * MAX_FORCE_AMPLITUDE;
        force_reaches[index] = rng.random_range(0.20..=1.0) * MAX_FORCE_REACH;
    });

    SporeConfigs {
        repulsion_dists,
        force_factors,
        force_reaches,
    }
}

pub fn generate_spores(nr_of_spores: u16) -> SporesState {
    let exp_nr_spores_per_bucket: usize = nr_of_spores as usize / NR_BUCKETS;

    let mut rng = rand::rng();

    // TODO: change to flat vec because inefficient!
    // see https://old.reddit.com/r/rust/comments/3bu7a8/access_time_of_vec_vs_array/cspq1bo/
    // see also: https://old.reddit.com/r/rust/comments/3bu7a8/access_time_of_vec_vs_array/csv37zl/
    // which say to use probably Matrix from NAlgebra, although my matrices are 3D, not 2D, with the lowest level dynamic sized
    let mut positions =
        vec![vec![Vec::with_capacity(exp_nr_spores_per_bucket); NR_HORZ_BUCKETS]; NR_VERT_BUCKETS];
    let mut speeds =
        vec![vec![Vec::with_capacity(exp_nr_spores_per_bucket); NR_HORZ_BUCKETS]; NR_VERT_BUCKETS];
    let mut spore_types =
        vec![vec![Vec::with_capacity(exp_nr_spores_per_bucket); NR_HORZ_BUCKETS]; NR_VERT_BUCKETS];

    for _ in 0..nr_of_spores {
        let x: f32 = rng.random_range(0.0..UNIVERSE_WIDTH);
        let y: f32 = rng.random_range(0.0..UNIVERSE_HEIGHT);

        let (horz, vert) = get_bucket(x, y);

        // println!("{}, {}", horz, vert);
        positions[vert][horz].push(Vector { x, y });
        speeds[vert][horz].push(ZERO_VECTOR);
        spore_types[vert][horz].push(rng.random_range(0..NUMBER_OF_CONFIGS as u8));
    }
    SporesState {
        positions,
        speeds,
        spore_types,
    }
}

pub const PREVIOUS_CONFIGS: SporeConfigs = SporeConfigs {
    repulsion_dists: [9.82, 5.97, 9.16, 17.59, 17.57, 5.08, 8.20, 10.20, 12.20],
    force_factors: [0.08, -0.04, 0.06, 0.07, 0.10, -0.08, 0.02, 0.04, -0.02],
    force_reaches: [
        27.41, 58.38, 38.75, 36.55, 52.39, 65.82, 30.05, 40.34, 45.98,
    ],
};
