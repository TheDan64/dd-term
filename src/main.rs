#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use conrod::{Labelable, Positionable, Sizeable, Colorable, Theme, Ui, Widget};
use conrod::color;
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};

fn main() {
    let window: PistonWindow = WindowSettings::new("Test window", [400, 270]).build().unwrap();

    for event in window.ups(30) {
    	println!("Hello, world!")
    }
}
