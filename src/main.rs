use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::WindowSettings;

extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

fn main() {
    let opengl = OpenGL::V3_2;
    let  _: GlutinWindow = WindowSettings::new("Pong", [512, 342])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);
    // gl.draw(viewport, f)
    println!("Hello, world!");
    loop {}
}
