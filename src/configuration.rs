// TODO put in config file

pub const MAX_REPULSION_DIST: f32 = 24.0;
pub const REPULSION_AMPLITUDE: f32 = -5.0 * MAX_FORCE_AMPLITUDE;
pub const MAX_FORCE_AMPLITUDE: f32 = 0.12;
pub const MAX_FORCE_REACH: f32 = 64.0;

// 64 = 40 * 1.6 -> 1.6 = SCREEN_RATIO
pub const NBR_VERT_BUCKETS: usize = 40;
pub const NBR_HORZ_BUCKETS: usize = 64; 
pub const NBR_BUCKETS: usize = NBR_HORZ_BUCKETS * NBR_VERT_BUCKETS;
pub const EXP_NBR_SPORES_PER_BUCKET: usize = 18;

pub const UNIVERSE_WIDTH: f32 = NBR_HORZ_BUCKETS as f32 * MAX_FORCE_REACH;
pub const UNIVERSE_HEIGHT: f32 = NBR_VERT_BUCKETS as f32 * MAX_FORCE_REACH;

pub const NUMBER_OF_CONFIGS: u8 = 6;

pub const NUMBER_OF_SPORES: u16 = 5000;

// friction should be low!
pub const FRICTION: f32 = 0.94;
