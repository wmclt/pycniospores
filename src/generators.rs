use crate::{
    bucket::get_bucket,
    configuration::{
        MAX_FORCE_AMPLITUDE, MAX_FORCE_REACH, MAX_REPULSION_DIST, NBR_OF_CONFS, NR_BUCKETS,
        NR_HORZ_BUCKETS, NR_VERT_BUCKETS, UNIVERSE_HEIGHT, UNIVERSE_WIDTH,
    },
    spore::{SporeConfigs, SporesState},
    vector::{Vector, ZERO_VECTOR},
};
use rand::prelude::*;

pub fn generate_spore_configs() -> SporeConfigs {
    // if USE_PREVIOUS_CONFIGURATIONS {
    //     return PREVIOUS_CONFIGS;
    // }

    let mut rng = rand::thread_rng();

    let mut repulsion_dists = [[0.0; NBR_OF_CONFS as usize]; NBR_OF_CONFS as usize];
    let mut force_factors = [[0.0; NBR_OF_CONFS as usize]; NBR_OF_CONFS as usize];
    let mut force_reaches = [[0.0; NBR_OF_CONFS as usize]; NBR_OF_CONFS as usize];

    (0..NBR_OF_CONFS as usize).for_each(|i| {
        (0..NBR_OF_CONFS as usize).for_each(|j| {
            repulsion_dists[i][j] = rng.gen_range(0.08..=1.5) * MAX_REPULSION_DIST;
            force_factors[i][j] = (rng.gen_range(0.55..=1.5) - 0.9)
                * MAX_FORCE_AMPLITUDE;
            force_reaches[i][j] = rng.gen_range(0.20..=1.0) * MAX_FORCE_REACH;
        });
    });

    SporeConfigs {
        repulsion_dists,
        force_factors,
        force_reaches,
    }
}

pub fn generate_spores(nr_of_spores: u16) -> SporesState {
    let exp_nr_spores_per_bucket: usize = nr_of_spores as usize / NR_BUCKETS;

    let mut rng = rand::thread_rng();

    let mut positions =
        vec![vec![Vec::with_capacity(exp_nr_spores_per_bucket); NR_HORZ_BUCKETS]; NR_VERT_BUCKETS];
    let mut speeds =
        vec![vec![Vec::with_capacity(exp_nr_spores_per_bucket); NR_HORZ_BUCKETS]; NR_VERT_BUCKETS];
    let mut spore_types =
        vec![vec![Vec::with_capacity(exp_nr_spores_per_bucket); NR_HORZ_BUCKETS]; NR_VERT_BUCKETS];

    for _ in 0..nr_of_spores {
        let x: f32 = rng.gen_range(0.0..UNIVERSE_WIDTH);
        let y: f32 = rng.gen_range(0.0..UNIVERSE_HEIGHT);

        let (horz, vert) = get_bucket(x, y);

        // println!("{}, {}", horz, vert);
        positions[vert][horz].push(Vector { x, y });
        speeds[vert][horz].push(ZERO_VECTOR);
        spore_types[vert][horz].push(rng.gen_range(0..NBR_OF_CONFS));
    }
    SporesState {
        positions,
        speeds,
        spore_types,
    }
}

// pub const PREVIOUS_CONFIGS: SporeConfigs = SporeConfigs {
//     repulsion_dists: [9.82, 5.97, 9.16, 17.59, 17.57, 5.08],
//     force_factors: [0.08, -0.04, 0.06, 0.07, 0.10, -0.08],
//     force_reaches: [27.41, 58.38, 38.75, 36.55, 52.39, 65.82],
// };
