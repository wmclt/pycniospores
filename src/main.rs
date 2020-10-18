extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

pub mod spore;

use std::f64::consts::PI;

use glutin_window::GlutinWindow;
use graphics::{circle_arc, clear, color::hex, Context, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderArgs, RenderEvent, WindowSettings};
use spore::{generate_spores, move_spores, Spore, SporeType, WINDOW_HEIGHT, WINDOW_WIDTH};

// TODO rename UNIVERSE_HEIGHT and UNIVERSE_WIDTH

const DIAMETER: f64 = 5.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Pycniospores", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        tick: 0,
        spores: generate_spores(WINDOW_WIDTH, WINDOW_HEIGHT),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // TODO resizing
        // if let Some(Button::Keyboard(key)) = e.press_args() {
        //     match key {
        //         piston::Key::Down => {
        //         }
        //         piston::Key::Up => {}
        //         _ => {}
        //     }
        // }

        if let Some(r) = e.render_args() {
            // TODO might be useful for window resizing: see https://github.com/amarao/piston_play/blob/e42099ae7dadff377e029af1703512f7765756da/src/main.rs
            // if r.draw_size[0] != x || r.draw_size[1] != y {
            //     let (draw_tx, draw_rx)= mpsc::sync_channel(128);

            //     let new_x = r.draw_size[0];
            //     let new_y = r.draw_size[1];
            //     println!("Resolution change from {}x{} to {}x{}", x, y, new_x, new_y);
            //     control_tx.send(ControlCommand{command: Command::NewResolution(new_x, new_y)}).unwrap();
            //     while let Ok(_command) = draw_rx.try_recv(){}
            //     buf = scale(buf, x, y, new_x, new_y);
            //     x = new_x;
            //     y = new_y;
            //     control_tx.send(ControlCommand{command:Command::Continue}).unwrap();
            //     texture = Texture::from_image(
            //         &mut texture_context,
            //         &buf,
            //         &TextureSettings::new()
            //     ).unwrap();
            // }
            app.tick += 1;
            move_spores(&mut app.spores);
            app.render(&r);
        }
    }
}

pub struct App {
    gl: GlGraphics,
    tick: u64,
    spores: Vec<Spore>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let background: [f32; 4] = hex("01011C");
        let spores = &self.spores;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(background, gl);

            // TODO show tick as text

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

// https://www.color-hex.com/color-palette/99210
pub fn get_color(sp: SporeType) -> [f32; 4] {
    match sp {
        // SporeType::One => [1.00, 1.00, 0.95, 1.0],
        SporeType::One => hex("BD000A"),
        // SporeType::Two => [0.04, 1.00, 0.42, 1.0],
        SporeType::Two => hex("F85E00"),
        // SporeType::Three => [0.08, 1.00, 0.50, 1.0],
        SporeType::Three => hex("1A936F"),
        // SporeType::Four => [0.11, 1.00, 0.50, 1.0],
        SporeType::Four => hex("FFA700"),
        // SporeType::Five => [0.57, 0.31, 0.72, 1.0],
        SporeType::Five => hex("A2BBCE"),
    }
}
