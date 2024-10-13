use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

use crate::layout::{NodeValue, Scopable};

pub struct Subtree<State: Scopable> {
    pub(crate) inner: ManuallyDrop<Box<NodeValue<State::Scoped>>>,
}

impl<T: Scopable> Drop for Subtree<T> {
    fn drop(&mut self) {
        // SAFETY: The field cannot be used after we drop
        unsafe { ManuallyDrop::drop(&mut self.inner) }
    }
}

impl<T: Scopable> Deref for Subtree<T> {
    type Target = NodeValue<T::Scoped>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Scopable> DerefMut for Subtree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Scopable> Clone for Subtree<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
