use crate::{
    bucket::get_buckets,
    configuration::{UNIVERSE_HEIGHT, UNIVERSE_WIDTH},
    generators::{generate_spore_configs, generate_spores},
    spore::{SporeConfigs, SporesState},
    spore_mover::move_spores,
    HEIGHT_RATIO, MAX_ZOOM, MIN_ZOOM, MOVE_INCREMENT, WIDTH_RATIO, ZOOM_SPEED,
};
use ggez::{
    self,
    context::Context,
    event,
    glam::Vec2,
    graphics::{self, Color, Mesh},
    input::keyboard::{KeyCode, KeyInput},
    mint::{Point2, Vector2},
    GameResult,
};

pub struct Simulation {
    nr_of_spores: u16,
    paused: bool,
    tick: u32,
    pub spore_configs: SporeConfigs,
    spores: SporesState,
    view_position: Point2<f32>,
    zoom: f32,
}

impl Simulation {
    pub fn new(ctx: &mut Context, nr_of_spores: u16) -> GameResult<Simulation> {
        ctx.gfx.add_font(
            "DejaVu",
            graphics::FontData::from_path(ctx, "/DejaVuSerif.ttf")?,
        );

        let s = Simulation {
            paused: false,
            nr_of_spores: nr_of_spores,
            tick: 0,
            spore_configs: generate_spore_configs(),
            spores: generate_spores(nr_of_spores),
            view_position: Point2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.paused {
            move_spores(&self.spore_configs, &mut self.spores);
            self.tick += 1;
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode.unwrap_or(KeyCode::A) {
            //just want it to shut up
            KeyCode::Escape => {
                ctx.request_quit();
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
                )
                .round();
                self.view_position.x = f32::max(
                    self.view_position.x,
                    -UNIVERSE_WIDTH * (self.zoom - WIDTH_RATIO),
                )
                .round();
            }
            KeyCode::Up => {
                let min_allowable_position_y = 0.0;
                self.view_position.y = f32::min(
                    min_allowable_position_y,
                    self.view_position.y + MOVE_INCREMENT,
                )
                .round();
            }
            KeyCode::Down => {
                // within bounds
                self.view_position.y = f32::max(
                    self.view_position.y - MOVE_INCREMENT,
                    -UNIVERSE_HEIGHT * (self.zoom - HEIGHT_RATIO),
                )
                .round();
            }
            KeyCode::Right => {
                // within bounds
                self.view_position.x = f32::max(
                    self.view_position.x - MOVE_INCREMENT,
                    -UNIVERSE_WIDTH * (self.zoom - WIDTH_RATIO),
                )
                .round();
            }
            KeyCode::Left => {
                let min_allowable_position_x = 0.0;
                self.view_position.x = f32::min(
                    min_allowable_position_x,
                    self.view_position.x + MOVE_INCREMENT,
                )
                .round();
            }
            _ => {}
        }
        Result::Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.tick % 8 == 0 {
            return Ok(());
        }

        let _background_color = rgb(34, 49, 63);

        // graphics::clear(ctx, background_color);

        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        draw_spores(ctx, &mut canvas, &self)?;
        show_numbers(
            ctx,
            &mut canvas,
            self.nr_of_spores,
            self.tick,
            self.zoom,
            self.view_position,
        )?;

        canvas.finish(ctx)?;

        // graphics::present(ctx)?;
        Ok(())
    }
}

fn draw_spores(
    ctx: &mut Context,
    canvas: &mut graphics::Canvas,
    universe: &Simulation,
) -> GameResult {
    let mut mesh_builder = graphics::MeshBuilder::new();
    for (horz, vert) in get_buckets() {
        let positions = &universe.spores.positions[vert][horz];
        let spore_types = &universe.spores.spore_types[vert][horz];
        for index in 0..(positions.len()) as usize {
            mesh_builder.circle(
                graphics::DrawMode::fill(),
                Vec2::new(positions[index].x, positions[index].y),
                4.0,
                0.01,
                get_color(spore_types[index]),
            )?;
        }
    }

    let mesh = mesh_builder.build();
    canvas.draw(
        &Mesh::from_data(&ctx.gfx, mesh),
        graphics::DrawParam::new()
            .scale(Vector2 {
                x: universe.zoom,
                y: universe.zoom,
            })
            .dest(universe.view_position),
    );

    Ok(())
}

fn get_color(spore_type: u8) -> Color {
    match spore_type {
        0 => rgb(238, 96, 85),  //red
        1 => rgb(220, 130, 27), // green
        2 => rgb(230, 210, 31), // light green
        3 => rgb(131, 221, 27), // orange
        4 => rgb(37, 186, 94),  // salmon
        5 => rgb(29, 227, 234), // blue
        6 => rgb(35, 96, 251),  // blue
        7 => rgb(169, 40, 243), // blue
        8 => rgb(230, 37, 237), // blue
        _ => panic!(),
    }
}

fn show_numbers(
    ctx: &mut Context,
    canvas: &mut graphics::Canvas,
    nr_of_spores: u16,
    tick: u32,
    zoom: f32,
    position: Point2<f32>,
) -> GameResult {
    // Text is drawn from the top-left corner.
    let offset = 10.0;
    let dest_point = ggez::glam::Vec2::new(offset, offset);
    canvas.draw(
        graphics::Text::new(
            format!(
                "#spores: {}\nTime: {}\nFPS: {:.2}\nTick: {}\nAVG ticks/s: {:.2}\nZoom: x{:.2}\nCoords: {:?}",
                nr_of_spores,
                format_duration(ctx.time.time_since_start().as_secs()),
                ctx.time.fps(),
                tick,
                (tick as f32) / ctx.time.time_since_start().as_secs_f32(),
                zoom,
                position,
            ),
        )
            .set_font("DejaVu")
            .set_scale(36.0),
        dest_point,
    );

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
