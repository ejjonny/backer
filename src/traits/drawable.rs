use crate::models::Area;

pub trait Drawable<State> {
    fn draw(&mut self, area: Area, state: &mut State);
}
