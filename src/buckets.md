
// max number of horiz buckets: 3840 / 70

UNIVERSE_HEIGHT: f32 = 2400.0;
UNIVERSE_WIDTH: f32 = 3840.0;


# DECISION: 
* make MAX_FORCE_REACH = 64 = 2**6;
* make UNIVERSE_WIDTH = 64 * 64 = 4096
* make UNIVERSE_HEIGHT = +/- 4096 * 2400 / 3840 = 40 * 64 = 2560
* the window dimensions are already set to 1.6

buckets will be organised in a matrix

# ORDER OF BUCKETS !!!
* always left to right
* then top to bottom
* like normal Western text