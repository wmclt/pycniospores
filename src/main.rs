use ggez::{self, event::quit, event::KeyCode, event::KeyMods, graphics::Color};
use ggez::{
    conf::{self},
    graphics, nalgebra as na,
};
use ggez::{event, graphics::Font};
use ggez::{
    nalgebra::{Point2, Vector2},
    timer,
};
use ggez::{Context, GameResult};

use std::{
    env,
    path::{self, PathBuf},
};

mod generators;
mod spore;
mod vector;
use generators::{generate_spore_configs, generate_spores};
use spore::{move_spores, SporeConfigs, Spores, NUMBER_OF_SPORES, UNIVERSE_HEIGHT, UNIVERSE_WIDTH};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

const HEIGHT_RATIO: f32 = WINDOW_HEIGHT / UNIVERSE_HEIGHT;
const WIDTH_RATIO: f32 = WINDOW_WIDTH / UNIVERSE_WIDTH;

const MAX_ZOOM: f32 = 4.0;
const MIN_ZOOM: f32 = HEIGHT_RATIO; // 1.0 Ideally should be const fn f32::min(HEIGHT_RATIO, WIDTH_RATIO)
const ZOOM_SPEED: f32 = 0.03;
const MOVE_INCREMENT: f32 = 40.0;

struct SporeUniverse {
    font: Font,
    paused: bool,
    tick: u32,
    spore_configs: SporeConfigs,
    spores: Spores,
    view_position: Point2<f32>,
    zoom: f32,
}

impl SporeUniverse {
    fn new(font: Font) -> GameResult<SporeUniverse> {
        let s = SporeUniverse {
            font,
            paused: false,
            tick: 0,
            spore_configs: generate_spore_configs(),
            spores: generate_spores(),
            view_position: Point2::new(0.0, 0.0),
            zoom: 1.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for SporeUniverse {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.paused {
            move_spores(&self.spore_configs, &mut self.spores);
            self.tick += 1;
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => {
                quit(ctx);
            }
            KeyCode::Space => {
                self.paused = !self.paused;
            }
            KeyCode::Comma => {
                //zoom in
                self.zoom = f32::min(MAX_ZOOM, self.zoom * (1.0 + ZOOM_SPEED));
            }
            KeyCode::Period => {
                // zoom out
                self.zoom = f32::max(MIN_ZOOM, self.zoom * (1.0 - ZOOM_SPEED));

                // replace within bounds
                self.view_position.y = f32::max(
                    self.view_position.y,
                    -UNIVERSE_HEIGHT * (self.zoom - HEIGHT_RATIO),
                ).round();
                self.view_position.x = f32::max(
                    self.view_position.x,
                    -UNIVERSE_WIDTH * (self.zoom - WIDTH_RATIO),
                ).round();
            }
            KeyCode::Up => {
                let min_allowable_position_y = 0.0;
                self.view_position.y = f32::min(
                    min_allowable_position_y,
                    self.view_position.y + MOVE_INCREMENT,
                ).round();
            }
            KeyCode::Down => {
                // within bounds
                self.view_position.y = f32::max(
                    self.view_position.y - MOVE_INCREMENT,
                    -UNIVERSE_HEIGHT * (self.zoom - HEIGHT_RATIO),
                ).round();
            }
            KeyCode::Right => {
                // within bounds
                self.view_position.x = f32::max(
                    self.view_position.x - MOVE_INCREMENT,
                    -UNIVERSE_WIDTH * (self.zoom - WIDTH_RATIO),
                ).round();
            }
            KeyCode::Left => {
                let min_allowable_position_x = 0.0;
                self.view_position.x = f32::min(
                    min_allowable_position_x,
                    self.view_position.x + MOVE_INCREMENT,
                ).round();
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = rgb(34, 49, 63);
        graphics::clear(ctx, background_color);

        draw_spores(ctx, &self)?;
        show_numbers(ctx, self.font, self.tick, self.zoom, self.view_position)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn draw_spores(ctx: &mut Context, universe: &SporeUniverse) -> GameResult {
    let mut mesh_builder = graphics::MeshBuilder::new();
    for index in 0..NUMBER_OF_SPORES as usize {
        mesh_builder.circle(
            graphics::DrawMode::fill(),
            na::Point2::new(
                universe.spores.positions[index].x,
                universe.spores.positions[index].y,
            ),
            4.0,
            0.01,
            get_color(universe.spores.spore_types[index]),
        );
    }
    let mesh = mesh_builder.build(ctx)?;
    graphics::draw(
        ctx,
        &mesh,
        graphics::DrawParam::new()
            .scale(Vector2::new(universe.zoom, universe.zoom))
            .dest(universe.view_position),
    )?;

    Ok(())
}

fn get_color(spore_type: u8) -> Color {
    match spore_type {
        0 => rgb(238, 96, 85),   //red
        1 => rgb(90, 200, 140),  // green
        2 => rgb(180, 250, 140), // light green
        3 => rgb(255, 217, 125), // orange
        4 => rgb(255, 155, 133), // salmon
        5 => rgb(89, 136, 207),  // blue
        _ => panic!(),
    }
}

fn show_numbers(
    ctx: &mut Context,
    font: Font,
    tick: u32,
    zoom: f32,
    position: Point2<f32>,
) -> GameResult {
    graphics::draw(
        ctx,
        &graphics::Text::new((
            format!(
                "#spores: {}\nTime: {}\nFPS: {:.2}\nTick: {}\nAVG ticks/s: {:.2}\nZoom: x{:.2}\nCoords: {}",
                NUMBER_OF_SPORES,
                format_duration(timer::time_since_start(ctx).as_secs()),
                timer::fps(ctx),
                tick,
                (tick as f32) / timer::time_since_start(ctx).as_secs_f32(),
                zoom,
                position,
            ),
            font,
            18.0,
        )),
        graphics::DrawParam::new()
            // .dest(dest_point)
            .color(graphics::WHITE),
    )?;
    Ok(())
}

fn format_duration(secs: u64) -> String {
    format!("{}m{}s", secs / 60, secs % 60)
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    [
        (r as f32) / 255.0,
        (g as f32) / 255.0,
        (b as f32) / 255.0,
        1.0,
    ]
    .into()
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

const WINDOW_HEIGHT: f32 = 800.0;
const WINDOW_WIDTH: f32 = 1280.0;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pycniospores", "william mclt")
        .add_resource_path(get_resource_dir())
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true),
        )
        .window_setup(conf::WindowSetup::default().title("pycniospores"));

    let (ctx, event_loop) = &mut cb.build()?;
    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
    let state = &mut SporeUniverse::new(font)?;

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
