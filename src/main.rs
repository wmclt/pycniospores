extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use rand::{distributions::Standard, prelude::*};
use std::f64::consts::PI;

use glutin_window::GlutinWindow;
use graphics::{circle_arc, clear, color::hex, Context, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderArgs, RenderEvent, WindowSettings};

// TODO rename UNIVERSE_HEIGHT and UNIVERSE_WIDTH
const WINDOW_HEIGHT: f64 = 800.0;
const WINDOW_WIDTH: f64 = 1200.0;

const DIAMETER: f64 = 5.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    println!("Hello, world!");

    let mut app = App {
        gl: GlGraphics::new(opengl),
        tick: 0,
        spores: generate_spores(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            // TODO exercise forces
            // TODO modify speed based on force - wrijving

            move_spores(&mut app.spores);

            app.render(&r);
        }
    }
}

fn move_spores(spores: &mut Vec<Spore>) {
    for spore in spores {
        spore.x_coord = (spore.x_coord + spore.x_speed) % WINDOW_WIDTH;
        spore.y_coord = (spore.y_coord + spore.y_speed) % WINDOW_HEIGHT;
    }
}

pub struct App {
    gl: GlGraphics,
    // left_score: i32,
    // left_pos: i32,
    // left_vel: i32,
    // right_score: i32,
    // right_pos: i32,
    // right_vel: i32,
    // ball_x: i32,
    // ball_y: i32,
    // vel_x: i32,
    // vel_y: i32,
    tick: u64,
    spores: Vec<Spore>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let background: [f32; 4] = hex("000033");

        let spores = &self.spores;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(background, gl);

            for spore in spores {
                draw_spore(c, gl, spore.spore_type, spore.x_coord, spore.y_coord);
            }

            // rectangle(FOREGROUND, rectangle::square(0.0, 0.0, 50.0), c.transform.trans(1.0, 1.0), gl);
        });
    }
}

fn draw_spore(c: Context, gl: &mut GlGraphics, sp: SporeType, x: f64, y: f64) {
    circle_arc(
        get_color(sp),
        DIAMETER / 2.0, //size/2.0 -> half-circle
        0.0,
        PI * 1.9999, //2.0*PI doesn't seem to work :l
        [DIAMETER, DIAMETER, DIAMETER, DIAMETER],
        c.transform.trans(x, y),
        gl,
    );
}

// colours: [red, green, blue, alpha]

// TODO colour field
#[derive(Debug, Copy, Clone)]
pub enum SporeType {
    One,
    Two,
    Three,
    Four,
    Five,
}
// https://www.color-hex.com/color-palette/99210
fn get_color(sp: SporeType) -> [f32; 4] {
    match sp {
        // SporeType::One => [1.00, 1.00, 0.95, 1.0],
        SporeType::One => hex("bd000a"),
        // SporeType::Two => [0.04, 1.00, 0.42, 1.0],
        SporeType::Two => hex("F85E00"),
        // SporeType::Three => [0.08, 1.00, 0.50, 1.0],
        SporeType::Three => hex("1A936F"),
        // SporeType::Four => [0.11, 1.00, 0.50, 1.0],
        SporeType::Four => hex("ffa700"),
        // SporeType::Five => [0.57, 0.31, 0.72, 1.0],
        SporeType::Five => hex("a2bbce"),
    }
}

pub struct Spore {
    x_coord: f64,
    y_coord: f64,
    x_speed: f64,
    y_speed: f64,
    spore_type: SporeType,
}

fn generate_spores() -> Vec<Spore> {
    let mut results = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let x_coord: f64 = rng.gen_range(0.0, WINDOW_WIDTH);
        let y_coord: f64 = rng.gen_range(0.0, WINDOW_HEIGHT);
        let x_speed: f64 = rng.gen_range(-1.0, 1.0);
        let y_speed: f64 = rng.gen_range(-1.0, 1.0);

        results.push(new_spore(
            x_coord,
            y_coord,
            x_speed,
            y_speed,
            rand::random(),
        ));
    }
    results
}

impl Distribution<SporeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SporeType {
        match rng.gen_range(0, 4) {
            0 => SporeType::One,
            1 => SporeType::Two,
            2 => SporeType::Three,
            3 => SporeType::Four,
            _ => SporeType::Five,
        }
    }
}

// TODO spores will start with speed 0
fn new_spore(
    x_coord: f64,
    y_coord: f64,
    x_speed: f64,
    y_speed: f64,
    spore_type: SporeType,
) -> Spore {
    Spore {
        x_coord,
        y_coord,
        x_speed,
        y_speed,
        spore_type,
    }
}
