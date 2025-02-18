// TODO put in config file

// DEFAULT WINDOW DIMENSIONS
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const WINDOW_WIDTH: f32 = 1280.0;

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
pub const MAX_FORCE_AMPLITUDE: f32 = 0.15;
pub const MAX_FORCE_REACH: f32 = 64.0 * 1.5;
// SPORE CONFIGS
pub const NUMBER_OF_CONFIGS: u8 = 6;
pub const NUMBER_OF_SPORES: u16 = 4096; // Can go up to 10k on a 2016 MacBook Pro

pub const FRICTION: f32 = 0.94; // friction should be low!

// BUCKETS
pub const BUCKET_SCALE_FACTOR: usize = 4;
pub const NR_VERT_BUCKETS: usize = 40 / BUCKET_SCALE_FACTOR; // 10
pub const NR_HORZ_BUCKETS: usize = 64 / BUCKET_SCALE_FACTOR; // 16
pub const BUCKET_HEIGHT: usize = UNIVERSE_HEIGHT as usize / NR_VERT_BUCKETS;
pub const BUCKET_WIDTH: usize = UNIVERSE_WIDTH as usize / NR_HORZ_BUCKETS;
pub const NR_BUCKETS: usize = NR_HORZ_BUCKETS * NR_VERT_BUCKETS; // 10*16 = 160
