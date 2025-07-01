use crate::game::{rendering, Game};
use crate::renderer::writers::{clear_screen, write_border, write_game};
use std::io::stdout;

pub mod consts;
pub mod macros;
pub mod writers;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }
}

impl rendering::Renderer for Renderer {
    fn render(&self, game: &Game) {
        let out = &mut stdout();
        write_game(out, game).expect("Failed to write game state");
    }

    fn prerender(&self) {
        let out = &mut stdout();
        clear_screen(out).expect("Failed to clear screen");
        write_border(out).expect("Failed to write border");
    }
}
