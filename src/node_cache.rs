use std::fmt::Debug;

use crate::{
    constraints::SizeConstraints,
    layout::NodeValue,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
};

pub(crate) struct NodeCache<State> {
    pub(crate) kind: NodeValue<State>,
    cache_area: Option<Area>,
    cached_constraints: Option<SizeConstraints>,
}

impl<State> NodeCache<State> {
    pub(crate) fn new(kind: NodeValue<State>) -> Self {
        Self {
            kind,
            cache_area: None,
            cached_constraints: None,
        }
    }
}

impl<State> Debug for NodeCache<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeCache")
            .field("kind", &self.kind)
            .field("cache_area", &self.cache_area)
            .field("cached_constraints", &self.cached_constraints)
            .finish()
    }
}

impl<State> NodeTrait<State> for NodeCache<State> {
    fn constraints(&mut self, available_area: Area, state: &mut State) -> SizeConstraints {
        if let (Some(cache), Some(constraints)) = (self.cache_area, self.cached_constraints) {
            if cache == available_area {
                return constraints;
            }
        }
        let constraints = self.kind.constraints(available_area, state);
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
    ) {
        self.kind.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            state,
        );
    }
    fn draw(&mut self, state: &mut State) {
        self.kind.draw(state)
    }
}
