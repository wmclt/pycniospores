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

const WINDOW_HEIGHT: f64 =  800.0;
const WINDOW_WIDTH: f64 = 1200.0;

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
            app.render(&r);
        }
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
        const BACKGROUND: [f32; 4] = [0.0, 0.1, 0.2, 0.8];

        let spores = &self.spores;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            for spore in spores {
                draw_circle(c, gl, spore.spore_type, spore.x_coord, spore.y_coord);
            }

            // draw_circle(c, gl, SporeType::One, 20.0, 40.0);
            // draw_circle(c, gl, SporeType::Two, 120.0, 140.0);
            // draw_circle(c, gl, SporeType::Three, 40.0, 80.0);
            // draw_circle(c, gl, SporeType::Four, 200.0, 140.0);
            // draw_circle(c, gl, SporeType::Five, 120.0, 66.0);

            // rectangle(FOREGROUND, rectangle::square(0.0, 0.0, 50.0), c.transform.trans(1.0, 1.0), gl);
        });
    }
}

fn draw_circle(c: Context, gl: &mut GlGraphics, sp: SporeType, x: f64, y: f64) {
    let size = 7.0;

    circle_arc(
        get_color(sp),
        size / 2.0, //size/2.0 -> half-circle
        0.0,
        PI * 1.9999, //PI * 3.999999999999 / 2.0, //2.0*PI doesn't seem to work :l
        [size, size, size, size],
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
        SporeType::Two => hex("d63600"),
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
    x_speed: u16,
    y_speed: u16,
    spore_type: SporeType,
}

fn generate_spores() -> Vec<Spore> {
    let mut results = Vec::new();
    let mut rng = rand::thread_rng();
    
    for _ in 0..100 {
        let x: f64 = rng.gen_range(0.0, WINDOW_WIDTH);
        let y: f64 = rng.gen_range(0.0, WINDOW_HEIGHT);
        results.push(new_spore(x, y, rand::random()));
    }
    results
}

impl Distribution<SporeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SporeType {
        match rng.gen_range(0, 3) {
            0 => SporeType::One,
            1 => SporeType::Two,
            _ => SporeType::Three,
        }
    }
}

fn new_spore(x_coord: f64, y_coord: f64, spore_type: SporeType) -> Spore {
    Spore {
        x_coord,
        y_coord,
        x_speed: 0,
        y_speed: 0,
        spore_type,
    }
}
