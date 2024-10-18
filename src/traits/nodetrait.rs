use crate::{constraints::SizeConstraints, models::Area};
use std::fmt::Debug;

pub(crate) trait NodeTrait<State>: Debug {
    fn draw(&mut self, state: &mut State);
    fn layout(&mut self, available_area: Area, state: &mut State);
    fn constraints(&mut self, area: Area, state: &mut State) -> SizeConstraints;
}
