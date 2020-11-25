use rand::prelude::*;

use crate::{
    spore::{
        SporeConfigs, Spores, DEFAULT_FORCE_AMPLITUDE, DEFAULT_FORCE_REACH, DEFAULT_REPULSION_DIST,
        NUMBER_OF_SPORES, UNIVERSE_HEIGHT, UNIVERSE_WIDTH,
    },
    vector::{Vector, ZERO_VECTOR},
};

const NUMBER_OF_CONFIGS: u8 = 6;

pub fn generate_spore_configs() -> SporeConfigs {
    let mut rng = rand::thread_rng();

    let mut repulsion_dists = Vec::with_capacity(6);
    let mut force_factors = Vec::with_capacity(6);
    let mut force_reaches = Vec::with_capacity(6);

    let randomly = false;
    if randomly {
        (0..NUMBER_OF_CONFIGS).for_each(|_| {
            repulsion_dists.push(rng.gen_range(0.08, 1.2) * DEFAULT_REPULSION_DIST);
            force_factors.push(
                rng.gen_range(0.15, 1.0)
                    * if rng.gen_bool(0.65) { 1.0 } else { -1.0 }
                    * DEFAULT_FORCE_AMPLITUDE,
            );
            force_reaches.push(rng.gen_range(0.20, 1.0) * DEFAULT_FORCE_REACH);
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

pub fn generate_spores() -> Spores {
    let mut rng = rand::thread_rng();

    let mut positions = Vec::with_capacity(NUMBER_OF_SPORES as usize);
    let mut speeds = Vec::with_capacity(NUMBER_OF_SPORES as usize);
    let mut spore_types = Vec::with_capacity(NUMBER_OF_SPORES as usize);

    for _ in 0..NUMBER_OF_SPORES {
        let x: f32 = rng.gen_range(0.0, UNIVERSE_WIDTH);
        let y: f32 = rng.gen_range(0.0, UNIVERSE_HEIGHT);

        positions.push(Vector { x, y });
        speeds.push(ZERO_VECTOR);
        spore_types.push(rng.gen_range(0, NUMBER_OF_CONFIGS));
    }
    Spores {
        positions,
        speeds,
        spore_types,
    }
}

// replace with previous spore configuration
fn get_previous_configs() -> SporeConfigs {
    SporeConfigs {
        repulsion_dists: vec![9.82, 5.97, 9.16, 17.59, 17.57, 5.08],
        force_factors: vec![0.08, -0.04, 0.06, 0.07, 0.10, -0.08],
        force_reaches: vec![27.41, 58.38, 38.75, 36.55, 52.39, 65.82],
    }
}
