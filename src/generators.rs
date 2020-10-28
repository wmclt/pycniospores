use std::collections::HashMap;

use rand::{distributions::Standard, prelude::*};

use crate::spore::{
    new_spore, Spore, SporeConfig, SporeConfigs, SporeType, DEFAULT_FORCE_AMPLITUDE,
    DEFAULT_FORCE_REACH, DEFAULT_REPULSION_DIST, NUMBER_OF_SPORES, WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub fn generate_spore_configs() -> SporeConfigs {
    let mut rng = rand::thread_rng();
    let mut configs = HashMap::new();

    // TODO move to config
    // decide here whether to generate random configs
    let randomly = true;
    if randomly {
        configs.insert(SporeType::One, generate_spore_config(&mut rng));
        configs.insert(SporeType::Two, generate_spore_config(&mut rng));
        configs.insert(SporeType::Three, generate_spore_config(&mut rng));
        configs.insert(SporeType::Four, generate_spore_config(&mut rng));
        configs.insert(SporeType::Five, generate_spore_config(&mut rng));
        configs.insert(SporeType::Six, generate_spore_config(&mut rng));
    } else {
        configs = get_previous_config();
    }

    configs
}

fn generate_spore_config(rng: &mut ThreadRng) -> SporeConfig {
    SporeConfig {
        repulsion_dist: rng.gen_range(0.25, 1.0) * DEFAULT_REPULSION_DIST,
        force_factor: rng.gen_range(0.25, 1.0)
            * if rng.gen_bool(0.65) { 1.0 } else { -1.0 }
            * DEFAULT_FORCE_AMPLITUDE,
        force_reach: rng.gen_range(0.3, 1.0) * DEFAULT_FORCE_REACH,
    }
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

// replace with previous spore configuration
fn get_previous_config() -> SporeConfigs {
    [
        (
            SporeType::One,
            SporeConfig {
                repulsion_dist: 5.78,
                force_factor: 0.03,
                force_reach: 62.78,
            },
        ),
        (
            SporeType::Two,
            SporeConfig {
                repulsion_dist: 15.26,
                force_factor: 0.04,
                force_reach: 39.70,
            },
        ),
        (
            SporeType::Three,
            SporeConfig {
                repulsion_dist: 10.20,
                force_factor: 0.06,
                force_reach: 41.02,
            },
        ),
        (
            SporeType::Four,
            SporeConfig {
                repulsion_dist: 9.32,
                force_factor: 0.08,
                force_reach: 31.57,
            },
        ),
        (
            SporeType::Five,
            SporeConfig {
                repulsion_dist: 16.29,
                force_factor: 0.04,
                force_reach: 54.99,
            },
        ),
        (
            SporeType::Six,
            SporeConfig {
                repulsion_dist: 11.90,
                force_factor: 0.12,
                force_reach: 49.15,
            },
        ),
    ]
    .iter()
    .cloned()
    .collect()
}
