use std::usize;

use crate::{configuration::NBR_OF_CONFS, vector::Vector};

type Buckets<T> = Vec<Vec<T>>;

pub struct SporesState {
    pub positions: Buckets<Vec<Vector>>,
    pub speeds: Buckets<Vec<Vector>>,
    pub spore_types: Buckets<Vec<u8>>,
}

// Uses Arrays instead of Vectors. Arrays are probably loaded to the CPU cache, while Vector has one more level of redirection.
#[derive(Debug)]
pub struct SporeConfigs {
    pub repulsion_dists: [[f32; NBR_OF_CONFS as usize]; NBR_OF_CONFS as usize],
    pub force_factors: [[f32; NBR_OF_CONFS as usize]; NBR_OF_CONFS as usize],
    pub force_reaches: [[f32; NBR_OF_CONFS as usize]; NBR_OF_CONFS as usize],
}
