use core::fmt;
use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

use crate::{
    constraints::{Constraint, SizeConstraints},
    models::Area,
    traits::{NodeTrait, ScopableOption},
    Node,
};

type SubtreeFn<'a, SubState> = Box<dyn Fn(&mut SubState) -> Node<'a, SubState> + 'a>;

pub(crate) struct Subtree<'a, SubState, State, StateScoper: ScopableOption<'a, State, SubState>> {
    pub(crate) subtree_fn: SubtreeFn<'a, SubState>,
    pub(crate) stored_tree: Option<Node<'a, SubState>>,
    pub(crate) _p: PhantomData<State>,
    pub(crate) _ss: PhantomData<StateScoper>,
}

impl<'a, SubState, State, StateScoper: ScopableOption<'a, State, SubState>> Debug
    for Subtree<'a, SubState, State, StateScoper>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subtree")
            .field("subtree_fn", &"<function>")
            .field("stored_tree", &self.stored_tree)
            .finish()
    }
}

impl<'a, SubState, State, StateScoper> NodeTrait<'a, State>
    for Subtree<'a, SubState, State, StateScoper>
where
    StateScoper: ScopableOption<'a, State, SubState>,
{
    fn draw(&mut self, state: &'a mut State) {
        StateScoper::scope_option(state, move |state| {
            let state = state?;
            let mut subtree = self.stored_tree.take().unwrap_or((self.subtree_fn)(state));
            subtree.inner.draw(state);
            self.stored_tree = Some(subtree);
            None::<()>
        });
    }
    fn layout(&mut self, available_area: Area, state: &'a mut State) {
        StateScoper::scope_option(state, move |state| {
            let state = state?;
            let mut subtree = self.stored_tree.take().unwrap_or((self.subtree_fn)(state));
            subtree.inner.layout(available_area, None, None, state);
            self.stored_tree = Some(subtree);
            None::<()>
        });
    }
    fn constraints(
        &mut self,
        area: Area,
        state: &'a mut State,
    ) -> crate::constraints::SizeConstraints {
        StateScoper::scope_option(state, move |state| {
            let state = state?;
            let mut subtree = self.stored_tree.take().unwrap_or((self.subtree_fn)(state));
            let result = subtree.inner.constraints(area, state);
            self.stored_tree = Some(subtree);
            Some(result)
        })
        .unwrap_or(SizeConstraints {
            width: Constraint::none(),
            height: Constraint::none(),
            aspect: None,
        })
    }
}
