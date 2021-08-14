//! Background color

use crate::{
    data::WidgetData,
    state::WidgetState,
    widgets::{
        utils::{
            decorator::WidgetDecorator,
            wrapper::{Wrapper, WrapperBindable},
        },
        Widget,
    },
};

pub trait BackgroundProperties {
    type Color;

    fn set_background_color(&mut self, color: Self::Color);
}

pub struct Background<W, P>
where
    P: BackgroundProperties,
{
    pub inner: W,
    pub background_properties: P,
    pub on_state_changed: fn(&mut Self, WidgetState),
}

impl<W, P> Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
    pub fn new(inner: W) -> Self
    where
        P: Default,
    {
        Background {
            background_properties: P::default(),
            inner,
            on_state_changed: |_, _| (),
        }
    }
}

impl<W, P> WrapperBindable for Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
}

impl<W, P> Background<W, P>
where
    P: BackgroundProperties,
{
    pub fn background_color(mut self, color: P::Color) -> Self {
        self.set_background_color(color);
        self
    }

    pub fn set_background_color(&mut self, color: P::Color) {
        self.background_properties.set_background_color(color);
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<W, P, D> Wrapper<Background<W, P>, D>
where
    W: Widget,
    P: BackgroundProperties,
    D: WidgetData,
{
    pub fn background_color(mut self, color: P::Color) -> Self {
        self.widget = self.widget.background_color(color);
        self
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Background<W, P>, WidgetState)) -> Self {
        // TODO this should be pulled up
        self.widget.on_state_changed = callback;
        self
    }
}

impl<W, P> WidgetDecorator for Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
    type Widget = W;

    fn widget(&self) -> &Self::Widget {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut Self::Widget {
        &mut self.inner
    }

    fn fire_on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
    }
}
