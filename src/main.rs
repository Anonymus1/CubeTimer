extern crate piston_window;
extern crate conrod;

use piston_window::*;
use conrod::{Labelable, Positionable, Sizeable, Colorable, Theme, Ui, Widget};
//use conrod::color;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Cube Timer", [1280, 720])
        .build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5,0.5,0.5,1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                     [0.0, 0.0, 500.0, 500.0], // rectangle
                     c.transform, g);
        });
    }
}
