use crate::vector::Vector;

type Buckets<T> = Vec<Vec<T>>;

pub struct SporesState {
    pub positions: Buckets<Vec<Vector>>,
    pub speeds: Buckets<Vec<Vector>>,
    pub spore_types: Buckets<Vec<u8>>,
}

#[derive(Debug)]
pub struct SporeConfigs {
    pub repulsion_dists: Vec<f32>,
    pub force_factors: Vec<f32>,
    pub force_reaches: Vec<f32>,
}
