use crate::{
    configuration::{
        BUCKET_HEIGHT, BUCKET_WIDTH, NBR_BUCKETS, NBR_HORZ_BUCKETS,
        NBR_VERT_BUCKETS,
    },
    vector::Vector,
};

// reading horizontally! => horizontal is interior loop
pub fn get_buckets() -> Vec<(usize, usize)> {
    let mut buckets = Vec::with_capacity(NBR_BUCKETS);
    for vert in 0..NBR_VERT_BUCKETS {
        for horz in 0..NBR_HORZ_BUCKETS {
            buckets.push((horz, vert));
        }
    }
    buckets
}

pub fn get_bucket(x: f32, y: f32) -> (usize, usize) {
    (
        (x / BUCKET_WIDTH as f32).floor() as usize,
        (y / BUCKET_HEIGHT as f32).floor() as usize,
    )
}

pub fn get_bucket_from_vec(pos: Vector) -> (usize, usize) {
    get_bucket(pos.x, pos.y)
}

// TODO iterator
pub fn get_neighbors(horz: usize, vert: usize) -> [(usize, usize); 9] {
    [
        (mod_horz(horz + NBR_HORZ_BUCKETS - 1), mod_vert(vert + 1)),
        (mod_horz(horz), mod_vert(vert + 1)),
        (mod_horz(horz + 1), mod_vert(vert + 1)),
        (mod_horz(horz + NBR_HORZ_BUCKETS - 1), mod_vert(vert)),
        (mod_horz(horz), mod_vert(vert)),
        (mod_horz(horz + 1), mod_vert(vert)),
        (
            mod_horz(horz + NBR_HORZ_BUCKETS - 1),
            mod_vert(vert + NBR_VERT_BUCKETS - 1),
        ),
        (mod_horz(horz), mod_vert(vert + NBR_VERT_BUCKETS - 1)),
        (mod_horz(horz + 1), mod_vert(vert + NBR_VERT_BUCKETS - 1)),
    ]
}

fn mod_horz(horz: usize) -> usize {
    horz % NBR_HORZ_BUCKETS
}

fn mod_vert(vert: usize) -> usize {
    vert % NBR_VERT_BUCKETS
}