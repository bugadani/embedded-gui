use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, Position},
    input::event::InputEvent,
    state::WidgetState,
    widgets::{
        wrapper::{Wrapper, WrapperBindable},
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

impl<W, P> Widget for Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
    fn attach(&mut self, parent: usize, self_index: usize) {
        self.inner.attach(parent, self_index);
    }

    fn arrange(&mut self, position: Position) {
        self.inner.arrange(position);
    }

    fn bounding_box(&self) -> BoundingBox {
        self.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        self.inner.bounding_box_mut()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec);
    }

    fn children(&self) -> usize {
        1 + self.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.inner
        } else {
            self.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.inner
        } else {
            self.inner.get_mut_child(idx - 1)
        }
    }

    fn parent_index(&self) -> usize {
        self.inner.parent_index()
    }

    fn update(&mut self) {
        self.inner.update();
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We just relay whatever the child desires
        self.inner.test_input(event).map(|i| i + 1)
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
        self.inner.on_state_changed(state);
    }

    fn is_selectable(&self) -> bool {
        false
    }
}
