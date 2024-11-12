use crate::models::Area;

pub(crate) trait Drawable<State> {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool);
}
