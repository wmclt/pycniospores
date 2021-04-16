// TODO put in config file

// UNIVERSE
// 64 = 40 * 1.6 -> 1.6 = SCREEN_RATIO
pub const UNIVERSE_SCALE_FACTOR: f32 = 2.0;
pub const UNIVERSE_WIDTH: f32 = 2560.0 * UNIVERSE_SCALE_FACTOR;
pub const UNIVERSE_HEIGHT: f32 = 1600.0 * UNIVERSE_SCALE_FACTOR;

// SPORE CONFIGURATIONS
pub const USE_PREVIOUS_CONFIGURATIONS: bool = false;

// SPORES
pub const MAX_REPULSION_DIST: f32 = 24.0;
pub const REPULSION_AMPLITUDE: f32 = -5.0 * MAX_FORCE_AMPLITUDE;
pub const MAX_FORCE_AMPLITUDE: f32 = 0.12;
pub const MAX_FORCE_REACH: f32 = 64.0;
// SPORE CONFIGS
pub const NUMBER_OF_CONFIGS: u8 = 6;
pub const NUMBER_OF_SPORES: u16 = 10_000;
// friction should be low!
pub const FRICTION: f32 = 0.94;

// BUCKETS
pub const BUCKET_SCALE_FACTOR: usize = 4;
pub const NBR_VERT_BUCKETS: usize = 40 / BUCKET_SCALE_FACTOR; // 10
pub const NBR_HORZ_BUCKETS: usize = 64 / BUCKET_SCALE_FACTOR; // 16
pub const BUCKET_HEIGHT: usize = UNIVERSE_HEIGHT as usize / NBR_VERT_BUCKETS;
pub const BUCKET_WIDTH: usize = UNIVERSE_WIDTH as usize / NBR_HORZ_BUCKETS;
pub const NBR_BUCKETS: usize = NBR_HORZ_BUCKETS * NBR_VERT_BUCKETS; // 10*16 = 160 
pub const EXP_NBR_SPORES_PER_BUCKET: usize = NUMBER_OF_SPORES as usize / NBR_BUCKETS;
