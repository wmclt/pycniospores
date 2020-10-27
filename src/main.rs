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
    collections::HashMap,
    env,
    path::{self, PathBuf},
};

pub mod spore;
use spore::{
    generate_spore_configs, generate_spores, move_spores, Spore, SporeConfig, SporeType,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

const MAX_ZOOM: f32 = 2.0;
const MIN_ZOOM: f32 = 1.0;
const ZOOM_SPEED: f32 = 0.02;
const MOVE_INCREMENT: f32 = 30.0;

struct SporeUniverse {
    font: Font,
    tick: u32,
    spore_configs: HashMap<SporeType, SporeConfig>,
    spores: Vec<Spore>,
    view_position: Point2<f32>,
    zoom: f32,
}

impl SporeUniverse {
    fn new(font: Font) -> GameResult<SporeUniverse> {
        let s = SporeUniverse {
            font,
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
        move_spores(&self.spore_configs, &mut self.spores);
        self.tick += 1;
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
            KeyCode::Comma => {
                //zoom in
                self.zoom = f32::min(MAX_ZOOM, self.zoom * (1.0 + ZOOM_SPEED));
            }
            KeyCode::Period => {
                // zoom out
                self.zoom = f32::max(MIN_ZOOM, self.zoom * (1.0 - ZOOM_SPEED));

                // replace within bounds
                self.view_position.x = f32::max(
                    self.view_position.x,
                    -WINDOW_WIDTH * (1.0 - (1.0 / self.zoom)),
                );
                self.view_position.y = f32::max(
                    self.view_position.y,
                    -WINDOW_HEIGHT * (1.0 - (1.0 / self.zoom)),
                );
            }
            KeyCode::Up => {
                let min_allowable_position_y = 0.0;
                self.view_position.y = f32::min(
                    min_allowable_position_y,
                    self.view_position.y + MOVE_INCREMENT,
                );
            }
            KeyCode::Down => {
                // within bounds
                self.view_position.y = f32::max(
                    self.view_position.y - MOVE_INCREMENT,
                    -WINDOW_HEIGHT * (1.0 - (1.0 / self.zoom)),
                );
            }
            KeyCode::Right => {
                // within bounds
                self.view_position.x = f32::max(
                    self.view_position.x - MOVE_INCREMENT,
                    -WINDOW_WIDTH * (1.0 - (1.0 / self.zoom)),
                );
            }
            KeyCode::Left => {
                let min_allowable_position_x = 0.0;
                self.view_position.x = f32::min(
                    min_allowable_position_x,
                    self.view_position.x + MOVE_INCREMENT,
                );
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = rgb(34, 49, 63);
        graphics::clear(ctx, background_color);

        draw_spores(ctx, &self)?;
        show_fps(ctx, self.font, self.tick, self.zoom, self.view_position)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn draw_spores(ctx: &mut Context, universe: &SporeUniverse) -> GameResult {
    let mut mesh_builder = graphics::MeshBuilder::new();
    for spore in &universe.spores {
        mesh_builder.circle(
            graphics::DrawMode::fill(),
            na::Point2::new(spore.x_coord, spore.y_coord),
            4.0,
            0.01,
            get_color(spore.spore_type),
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

fn get_color(spore_type: SporeType) -> Color {
    match spore_type {
        SporeType::One => rgb(238, 96, 85),     //red
        SporeType::Two => rgb(96, 211, 148),    // green
        SporeType::Three => rgb(170, 246, 131), // light green
        SporeType::Four => rgb(255, 217, 125),  // orange
        SporeType::Five => rgb(255, 155, 133),  // salmon
        SporeType::Six => rgb(89, 136, 207),    // blue
    }
}

fn show_fps(
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
                "FPS: {:.2}\nTick: {}\nZoom: x{:.2}\nCoords: {}",
                timer::fps(ctx),
                tick,
                zoom,
                position,
            ),
            font,
            24.0,
        )),
        graphics::DrawParam::new()
            // .dest(dest_point)
            .color(graphics::WHITE),
    )?;
    Ok(())
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

    println!("Spore configuration:\n {:?}", state.spore_configs);
    event::run(ctx, event_loop, state)
}
