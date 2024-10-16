use crate::{constraints::SizeConstraints, models::Area};
use std::fmt::Debug;

pub(crate) trait NodeTrait<State, Ctx>: Debug {
    fn draw(&mut self, state: &mut State, ctx: &mut Ctx);
    fn layout(&mut self, available_area: Area, state: &mut State, ctx: &mut Ctx);
    fn constraints(&mut self, area: Area, state: &mut State, ctx: &mut Ctx) -> SizeConstraints;
}
