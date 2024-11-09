use std::fmt::Debug;

use crate::{
    constraints::SizeConstraints,
    layout::NodeValue,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
};

pub(crate) struct NodeCache<State, Ctx> {
    pub(crate) kind: NodeValue<State, Ctx>,
    cache_area: Option<Area>,
    cached_constraints: Option<SizeConstraints>,
}

impl<State, Ctx> NodeCache<State, Ctx> {
    pub(crate) fn new(kind: NodeValue<State, Ctx>) -> Self {
        Self {
            kind,
            cache_area: None,
            cached_constraints: None,
        }
    }
}

impl<State, Ctx> Debug for NodeCache<State, Ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeCache")
            .field("kind", &self.kind)
            .field("cache_area", &self.cache_area)
            .field("cached_constraints", &self.cached_constraints)
            .finish()
    }
}

impl<State, Ctx> NodeTrait<State, Ctx> for NodeCache<State, Ctx> {
    fn constraints(
        &mut self,
        available_area: Area,
        state: &mut State,
        ctx: &mut Ctx,
    ) -> SizeConstraints {
        if let (Some(cache), Some(constraints)) = (self.cache_area, self.cached_constraints) {
            if cache == available_area {
                return constraints;
            }
        }
        let constraints = self.kind.constraints(available_area, state, ctx);
        self.cache_area = Some(available_area);
        self.cached_constraints = Some(constraints);
        constraints
    }
    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
        ctx: &mut Ctx,
    ) {
        self.kind.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            state,
            ctx,
        )
    }
    fn draw(&mut self, state: &mut State, ctx: &mut Ctx) {
        self.kind.draw(state, ctx)
    }
}
