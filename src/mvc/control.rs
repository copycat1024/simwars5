use super::{Dispatch, Model};
use crate::view::{Compose, NodeRef};
use soyo::tui::Event;

pub struct Control<M, C>
where
    M: Model,
    C: Compose,
{
    dispatch: fn(Event, &NodeRef<C>, &mut Dispatch<M::Event>),
    update: fn(&M, &mut NodeRef<C>),
}

impl<M, C> Control<M, C>
where
    M: Model,
    C: Compose,
{
    pub const fn new(
        dispatch: fn(Event, &NodeRef<C>, &mut Dispatch<M::Event>),
        update: fn(&M, &mut NodeRef<C>),
    ) -> Self {
        Self { dispatch, update }
    }

    pub fn dispatch(&self, event: Event, view: &NodeRef<C>, dispatch: &mut Dispatch<M::Event>) {
        (self.dispatch)(event, view, dispatch);
    }

    pub fn update(&self, model: &M, view: &mut NodeRef<C>) {
        (self.update)(model, view);
    }
}
