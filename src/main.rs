extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate game_of_life;
mod patterns;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use patterns::*;
use game_of_life::*;

struct App {
    gl: GlGraphics,
    world: Vec<Vec<bool>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const SCALE: usize = 3;

        let mut squares = Vec::new();
        
        for &(x, y) in get_alive_cells(&self.world).iter() {
            squares.push(rectangle::square((x * SCALE) as f64, (y * SCALE) as f64, SCALE as f64));
        }

        //let (x, y) = (arg.width / 2.0, args.height / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen
            clear(BLACK, gl);

            let transform = c.transform;

            //Draw a box
            for square in squares.iter() {
                rectangle(WHITE, *square, transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        //TO-DO
        //update
        //let (lives, dies) = get_state(&self.world);
        //change_state(&mut self.world, lives, dies);
        next_generation(&mut self.world);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Conway's Game of Life",
        [1050, 450]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: init_world(150, 350),
    };
    
    insert_pattern(&mut app.world, &get_pattern(Patterns::Toad), 20, 10);
    insert_pattern(&mut app.world, &get_pattern(Patterns::Beacon), 20, 0);
    insert_pattern(&mut app.world, &get_pattern(Patterns::Pulsar), 0, 0);
    insert_pattern(&mut app.world, &get_pattern(Patterns::Blinker), 76, 0);
    insert_pattern(&mut app.world, &get_pattern(Patterns::GosperGliderGun), 0, 20);

    let mut event_settings = EventSettings::new();
    event_settings.set_ups(10);
    let mut events = Events::new(event_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }        
    }       
}