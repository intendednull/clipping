extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod gh;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::polygon;
use graphics::types::Vec2d;

fn render(args: &RenderArgs, gl: &mut GlGraphics, poly_a: &Vec<Vec2d>, poly_b: &Vec<Vec2d>, poly_c: &Vec<Vec<Vec2d>>) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    gl.draw(args.viewport(), |c, gl|{
        clear(BLACK, gl);

        let trans = c.transform.trans(0., 0.);
        //polygon(RED, &poly_a, trans, gl);
        //polygon(BLUE, &poly_b, trans, gl);

        for clip in poly_c{
            //let trans = c.transform.rot_rad(-1.570796).trans(-200., 0.);
            polygon(GREEN, &clip, trans, gl);
        }

    });
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "clipping",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let polyA: Vec<Vec2d> = vec![[40., 34.], [200., 66.], [106., 80.], [120., 175.]];
    let polyB = vec![[133., 120.], [80., 146.], [26., 106.], [40., 90.], [0., 53.], [80., 66.], [146., 0.]];

    let mut cpA = gh::CPolygon::from_vec(&polyA);
    let mut cpB = gh::CPolygon::from_vec(&polyB);

    let cpAB = cpA.intersection(&mut cpB);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            render(&r, &mut gl, &polyA, &polyB, &cpAB);
        }
    }
}
