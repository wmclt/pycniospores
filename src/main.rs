use clap::{App, Arg};
use configuration::{
    NUMBER_OF_SPORES, UNIVERSE_HEIGHT, UNIVERSE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use ggez::{
    self,
    conf::{self},
    event, graphics, GameResult,
};
use simulation::Simulation;
use std::{
    env,
    path::{self, PathBuf},
    str::FromStr,
};

mod bucket;
mod configuration;
mod generators;
mod movement_calculator;
mod simulation;
mod spore;
mod spore_mover;
mod vector;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

const HEIGHT_RATIO: f32 = WINDOW_HEIGHT / UNIVERSE_HEIGHT;
const WIDTH_RATIO: f32 = WINDOW_WIDTH / UNIVERSE_WIDTH;

const MAX_ZOOM: f32 = 4.0;
const MIN_ZOOM: f32 = HEIGHT_RATIO; // 1.0 Ideally should be const fn f32::min(HEIGHT_RATIO, WIDTH_RATIO)
const ZOOM_SPEED: f32 = 0.03;
const MOVE_INCREMENT: f32 = 40.0;

pub fn main() -> GameResult {
    let nr_of_spores = get_nr_of_spores();

    let cb = ggez::ContextBuilder::new("Pycniospores", "Pycniospores")
        .add_resource_path(get_resource_dir())
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        )
        .window_setup(conf::WindowSetup::default().title("Pycniospores"));

    let (mut ctx, event_loop) = cb.build()?;
    let font = graphics::Font::new(&mut ctx, "/DejaVuSerif.ttf")?;
    let state = Simulation::new(font, nr_of_spores)?;

    println!(
        "\nWelcome to Pycniospores! A spores simulator.\n
        ,\tto zoom in\n
        .\tto zoom out\n
        arrows\tto move around\n
        space\tto pause\n
        esc\tto quit\n\n
        Spore configuration:\n\n {}\n",
        format!("{:.2?}", state.spore_configs)
    );
    event::run(ctx, event_loop, state)
}

// TODO update to clap v3 when available
// TODO also read if random configs or not
fn get_nr_of_spores() -> u16 {
    let matches = App::new("Pycniospores")
        .version("0.1")
        .author("@wmclt on Gitlab")
        .about("The large particle simulator")
        .arg(
            Arg::with_name("number")
                .help("Sets the number of particles ('spores')")
                .short("n")
                .long("number")
                .takes_value(true),
        )
        .get_matches();
    let nr_of_spores = matches
        .value_of("number")
        .map(|s| FromStr::from_str(s))
        .map(|a| a.unwrap())
        .unwrap_or(NUMBER_OF_SPORES);
    nr_of_spores
}

fn get_resource_dir() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    }
}
