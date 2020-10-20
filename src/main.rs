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

struct MainState {
    tick: u64,
    spores: Vec<Spore>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            tick: 0,
            spores: generate_spores(800.0, 800.0),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        move_spores(&mut self.spores);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = rgb(34, 49, 63);
        graphics::clear(ctx, background_color);

        self.spores
            .iter()
            .map(|spore| draw_spore(ctx, &spore))
            .for_each(|r| r.unwrap());

        show_fps(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn show_fps(ctx: &mut Context) -> GameResult{
    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
    graphics::draw(
        ctx,
        &graphics::Text::new((format!("{}", timer::fps(ctx)), font, 48.0)),
        graphics::DrawParam::new()
            // .dest(dest_point)
            .color(graphics::WHITE),
    )?;
    Ok(())
}

fn draw_spore(ctx: &mut Context, spore: &Spore) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(0.0, 0.0),
        5.0,
        0.01,
        get_color(spore.spore_type),
    )?;
    graphics::draw(
        ctx,
        &circle,
        (na::Point2::new(spore.x_coord, spore.y_coord),),
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
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
