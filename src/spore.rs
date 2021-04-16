use std::usize;

use crate::{configuration::NUMBER_OF_CONFIGS, vector::Vector};

type Buckets<T> = Vec<Vec<T>>;

pub struct SporesState {
    pub positions: Buckets<Vec<Vector>>,
    pub speeds: Buckets<Vec<Vector>>,
    pub spore_types: Buckets<Vec<u8>>,
}

// Uses Arrays instead of Vectors. Arrays are probably loaded to the CPU cache, while Vector has one more level of redirection.
#[derive(Debug)]
pub struct SporeConfigs {
    pub repulsion_dists: [f32; NUMBER_OF_CONFIGS as usize],
    pub force_factors: [f32; NUMBER_OF_CONFIGS as usize],
    pub force_reaches: [f32; NUMBER_OF_CONFIGS as usize],
}
