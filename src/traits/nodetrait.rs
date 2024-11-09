use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
};
use std::fmt::Debug;

pub(crate) trait NodeTrait<State, Ctx>: Debug {
    fn constraints(
        &mut self,
        available_area: Area,
        state: &mut State,
        ctx: &mut Ctx,
    ) -> SizeConstraints;
    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
        ctx: &mut Ctx,
    );
    fn draw(&mut self, state: &mut State, ctx: &mut Ctx);
}
