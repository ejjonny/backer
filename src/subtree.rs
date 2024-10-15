use core::fmt;
use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

use crate::{
    constraints::{Constraint, SizeConstraints},
    models::Area,
    traits::{NodeTrait, ScopableOption},
    NodeWith,
};

type SubtreeFn<SubState, SubCtx> =
    Box<dyn Fn(&mut SubState, &mut SubCtx) -> NodeWith<SubState, SubCtx>>;

pub(crate) struct Subtree<
    SubState,
    SubCtx,
    State: ScopableOption<SubState>,
    Ctx: ScopableOption<SubCtx>,
> {
    pub(crate) subtree_fn: SubtreeFn<SubState, SubCtx>,
    pub(crate) stored_tree: Option<NodeWith<SubState, SubCtx>>,
    pub(crate) _p: PhantomData<State>,
    pub(crate) _c: PhantomData<Ctx>,
}

impl<SubState, SubCtx, State: ScopableOption<SubState>, Ctx: ScopableOption<SubCtx>> Debug
    for Subtree<SubState, SubCtx, State, Ctx>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subtree")
            .field("subtree_fn", &"<function>")
            .field("stored_tree", &self.stored_tree)
            .finish()
    }
}

impl<SubCtx, SubState, State, Ctx> NodeTrait<State, Ctx> for Subtree<SubState, SubCtx, State, Ctx>
where
    State: ScopableOption<SubState>,
    Ctx: ScopableOption<SubCtx>,
{
    fn draw(&mut self, state: &mut State, ctx: &mut Ctx) {
        state.scope_option(|state| {
            ctx.scope_option(|ctx| {
                let (Some(state), Some(ctx)) = (state, ctx) else {
                    return None::<()>;
                };
                let mut subtree = self
                    .stored_tree
                    .take()
                    .unwrap_or((self.subtree_fn)(state, ctx));
                subtree.inner.draw(state, ctx);
                self.stored_tree = Some(subtree);
                None::<()>
            })
        });
    }
    fn layout(&mut self, available_area: Area, state: &mut State, ctx: &mut Ctx) {
        state.scope_option(|state| {
            ctx.scope_option(|ctx| {
                let (Some(state), Some(ctx)) = (state, ctx) else {
                    return None::<()>;
                };
                let mut subtree = self
                    .stored_tree
                    .take()
                    .unwrap_or((self.subtree_fn)(state, ctx));
                subtree.inner.layout(available_area, None, None, state, ctx);
                self.stored_tree = Some(subtree);
                None::<()>
            })
        });
    }
    fn constraints(
        &mut self,
        area: Area,
        state: &mut State,
        ctx: &mut Ctx,
    ) -> crate::constraints::SizeConstraints {
        state
            .scope_option(|state| {
                ctx.scope_option(|ctx| {
                    let (Some(state), Some(ctx)) = (state, ctx) else {
                        return None;
                    };
                    let mut subtree = self
                        .stored_tree
                        .take()
                        .unwrap_or((self.subtree_fn)(state, ctx));
                    let result = subtree.inner.constraints(area, state, ctx);
                    self.stored_tree = Some(subtree);
                    Some(result)
                })
            })
            .unwrap_or(SizeConstraints {
                width: Constraint::none(),
                height: Constraint::none(),
                aspect: None,
            })
    }
}
