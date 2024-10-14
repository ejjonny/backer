use std::marker::PhantomData;

use crate::{models::Area, traits::NodeTrait, traits::Scopable, Node};

type SubtreeFn<SubState> = Box<dyn Fn(SubState) -> Node<SubState>>;
pub(crate) struct Subtree<SubState: Copy, State: Scopable<SubState>> {
    pub(crate) subtree_fn: SubtreeFn<SubState>,
    pub(crate) stored_tree: Option<Node<SubState>>,
    pub(crate) _p: PhantomData<State>,
}

impl<SubState: Copy, State: Scopable<SubState> + Copy> NodeTrait<State>
    for Subtree<SubState, State>
{
    fn draw(&mut self, state: State) {
        State::scope(state, |s| {
            let mut subtree = self.stored_tree.take().unwrap_or((self.subtree_fn)(s));
            subtree.inner.draw(s);
            self.stored_tree = Some(subtree);
        })
    }
    fn layout(&mut self, available_area: Area, state: State) {
        State::scope(state, |s| {
            let mut subtree = self.stored_tree.take().unwrap_or((self.subtree_fn)(s));
            subtree.inner.layout(available_area, None, None, s);
            self.stored_tree = Some(subtree);
        })
    }
    fn constraints(&mut self, area: Area, state: State) -> crate::constraints::SizeConstraints {
        State::scope(state, |s| {
            let mut subtree = self.stored_tree.take().unwrap_or((self.subtree_fn)(s));
            let result = subtree.inner.constraints(area, s);
            self.stored_tree = Some(subtree);
            result
        })
    }
}
