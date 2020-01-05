extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate timer;
extern crate chrono;

use crate::piston::MouseCursorEvent;
use crate::piston::PressEvent;
use rand::{thread_rng, Rng};
use std::sync::mpsc::channel;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the rect.
    last_dt: f64,
    total_time: f64,
    background_color: [f32; 4],
    rect_color: [f32; 4],
    cursor_pos: [f64; 2],
}

impl App 
{
    fn render(&mut self, args: &RenderArgs) 
    {
        use graphics::*;

        let rotation = self.rotation;
        let (x, y) = (self.cursor_pos[0], self.cursor_pos[1]);
        let (w, h) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        
        let rect = rectangle::rectangle_by_corners(x-w/2.0, y-h/2.0, x+w/2.0, y+h/2.0);

        let rect_color = self.rect_color;
        let background_color = self.background_color;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(background_color, gl);

            let transform = c.transform
                .rot_rad(rotation);

            // Draw a box rotating around the middle of the screen.
            rectangle(rect_color, rect, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) 
    {
        self.total_time += args.dt;

        let dif = self.total_time - self.last_dt; 
        if dif > 2.0
        {
            self.last_dt = self.total_time;
                        
            let mut rng = thread_rng();
            let f1: f32 = rng.gen_range(0.0, 1.0);
            let f2: f32 = rng.gen_range(0.0, 1.0);
            let f3: f32 = rng.gen_range(0.0, 1.0);
            let f4: f32 = rng.gen_range(0.0, 1.0);
            let f5: f32 = rng.gen_range(0.0, 1.0);
            let f6: f32 = rng.gen_range(0.0, 1.0);

            self.rect_color = [f1, f2, f3, 1.0];
            self.background_color = [f4, f5, f6, 1.0];
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-rect", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        last_dt: 0.0,
        total_time: 0.0,
        background_color: [0.0, 1.0, 0.0, 1.0],
        rect_color: [1.0, 0.0, 0.0, 1.0],
        cursor_pos: [0.0, 0.0],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) 
    {        
        if let Some(pos) = e.mouse_cursor_args() {
            app.cursor_pos = pos;
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}