use crate::game::Game;

pub trait Renderer: Send + Sync {
    fn render(&self, game: &Game);
    fn prerender(&self);
}
