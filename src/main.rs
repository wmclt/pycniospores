//! The simplest possible example that does something.

use std::{env, path};

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{self, graphics::Color};
use ggez::{Context, GameResult};
use spore::{generate_spores, move_spores, Spore, SporeType};

pub mod spore;

struct SporeUniverse {
    tick: u32,
    spores: Vec<Spore>,
}

impl SporeUniverse {
    fn new() -> GameResult<SporeUniverse> {
        let s = SporeUniverse {
            tick: 0,
            spores: generate_spores(),
        };
        Ok(s)
    }
}

impl event::EventHandler for SporeUniverse {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        move_spores(&mut self.spores);
        self.tick += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = rgb(34, 49, 63);
        graphics::clear(ctx, background_color);

        draw_spores(ctx, &self.spores)?;
        show_fps(ctx, self.tick)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn show_fps(ctx: &mut Context, tick: u32) -> GameResult {
    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
    graphics::draw(
        ctx,
        &graphics::Text::new((
            format!("FPS: {} \nTick: {}", timer::fps(ctx), tick),
            font,
            24.0,
        )),
        graphics::DrawParam::new()
            // .dest(dest_point)
            .color(graphics::WHITE),
    )?;
    Ok(())
}

fn draw_spores(ctx: &mut Context, spores: &Vec<Spore>) -> GameResult {
    let mut mesh_builder = graphics::MeshBuilder::new();
    for spore in spores {
        mesh_builder.circle(
            graphics::DrawMode::fill(),
            na::Point2::new(spore.x_coord, spore.y_coord),
            5.0,
            0.01,
            get_color(spore.spore_type),
        );
    }
    let mesh = mesh_builder.build(ctx)?;
    graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

    Ok(())
}

fn get_color(spore_type: SporeType) -> Color {
    match spore_type {
        SporeType::One => rgb(238, 96, 85),     //red
        SporeType::Two => rgb(96, 211, 148),    // green
        SporeType::Three => rgb(170, 246, 131), // light green
        SporeType::Four => rgb(255, 217, 125),  // orange
        SporeType::Five => rgb(255, 155, 133),  // salmon
    }
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

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let cb =
        ggez::ContextBuilder::new("pycniospores", "william mclt").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut SporeUniverse::new()?;
    event::run(ctx, event_loop, state)
}
