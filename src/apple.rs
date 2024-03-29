use crate::render::render_block;
use crate::utils::Drawable;
use crate::map::Location;
use crate::consts::*;


pub struct Apple {

    pub location: Location,

}


impl Apple {

    pub fn new(location: Location) -> Self {
        Apple {
            location
        }
    }

}


impl Drawable for Apple {

    fn draw(&mut self, args: &piston::RenderArgs, gl: &mut opengl_graphics::GlGraphics, _window: &mut piston_window::PistonWindow, _event: &piston::Event) {
        gl.draw(args.viewport(), |context, gl| {
            render_block(APPLE_COLOR, self.location, &context, gl);
        });
    }

}

