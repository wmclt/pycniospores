use crate::{
    bucket::get_buckets,
    configuration::{UNIVERSE_HEIGHT, UNIVERSE_WIDTH},
    generators::{generate_spore_configs, generate_spores},
    spore::{SporeConfigs, SporesState},
    spore_mover::move_spores,
    HEIGHT_RATIO, MAX_ZOOM, MIN_ZOOM, MOVE_INCREMENT, WIDTH_RATIO, ZOOM_SPEED,
};
use ggez::{
    self, event,
    event::quit,
    event::KeyCode,
    event::KeyMods,
    graphics,
    graphics::Color,
    graphics::Font,
    mint::{Point2, Vector2},
    timer, Context, GameResult,
};

pub struct Simulation {
    font: Font,
    nr_of_spores: u16,
    paused: bool,
    tick: u32,
    pub spore_configs: SporeConfigs,
    spores: SporesState,
    view_position: Point2<f32>,
    zoom: f32,
}

impl Simulation {
    pub fn new(font: Font, nr_of_spores: u16) -> GameResult<Simulation> {
        let s = Simulation {
            font,
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
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = rgb(34, 49, 63);
        graphics::clear(ctx, background_color);

        draw_spores(ctx, &self)?;
        show_numbers(
            ctx,
            self.font,
            self.nr_of_spores,
            self.tick,
            self.zoom,
            self.view_position,
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn draw_spores(ctx: &mut Context, universe: &Simulation) -> GameResult {
    let mut mesh_builder = graphics::MeshBuilder::new();
    for (horz, vert) in get_buckets() {
        let positions = &universe.spores.positions[vert][horz];
        let spore_types = &universe.spores.spore_types[vert][horz];
        for index in 0..(positions.len()) as usize {
            mesh_builder.circle(
                graphics::DrawMode::fill(),
                Point2 {
                    x: positions[index].x,
                    y: positions[index].y,
                },
                4.0,
                0.01,
                get_color(spore_types[index]),
            )?;
        }
    }

    let mesh = mesh_builder.build(ctx)?;
    graphics::draw(
        ctx,
        &mesh,
        graphics::DrawParam::new()
            .scale(Vector2 {
                x: universe.zoom,
                y: universe.zoom,
            })
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
    nr_of_spores: u16,
    tick: u32,
    zoom: f32,
    position: Point2<f32>,
) -> GameResult {
    graphics::draw(
        ctx,
        &graphics::Text::new((
            format!(
                "#spores: {}\nTime: {}\nFPS: {:.2}\nTick: {}\nAVG ticks/s: {:.2}\nZoom: x{:.2}\nCoords: {:?}",
                nr_of_spores,
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
            .color(Color::WHITE),
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
