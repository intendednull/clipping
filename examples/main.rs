extern crate piston_window;
extern crate clipping;

use clipping::gh;

use piston_window::*;

fn render<E: GenericEvent>(e: &E, gl: &mut PistonWindow, poly_a: &Vec<[f64; 2]>, poly_b: &Vec<[f64; 2]>, poly_c: &Vec<Vec<[f64; 2]>>) {

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    gl.draw_2d(e, |c, gl|{
        clear(BLACK, gl);

        let trans = c.transform.trans(0., 0.);
        polygon(RED, &poly_a, trans, gl);
        polygon(BLUE, &poly_b, trans, gl);

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
    let mut window: PistonWindow = WindowSettings::new(
            "clipping",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let poly_a: Vec<[f64; 2]> = vec![[40., 34.], [200., 66.], [106., 80.], [120., 175.]];
    let poly_b = vec![[133., 120.], [80., 146.], [26., 106.], [40., 90.], [0., 53.], [80., 66.], [146., 0.]];

    let mut cp_a = gh::CPolygon::from_vec(&poly_a);
    let mut cp_b = gh::CPolygon::from_vec(&poly_b);

    let cp_ab = cp_a.intersection(&mut cp_b);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        render(&e, &mut window, &poly_a, &poly_b, &cp_ab);
    }
}
