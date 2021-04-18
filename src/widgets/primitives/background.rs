use crate::{
    data::WidgetData,
    geometry::{BoundingBox, Position},
    input::event::InputEvent,
    widgets::{wrapper::Wrapper, ParentHolder, UpdateHandler, Widget, WidgetStateHolder},
    MeasureSpec, WidgetState,
};

pub trait BackgroundProperties {
    type Color;

    fn set_background_color(&mut self, color: Self::Color) -> &mut Self;
}

pub struct Background<W, P>
where
    P: BackgroundProperties,
{
    pub inner: W,
    pub background_properties: P,
    pub parent_index: usize,
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub state: WidgetState,
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
            parent_index: 0,
            background_properties: P::default(),
            inner,
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }

    pub fn background_color(mut self, color: P::Color) -> Self {
        self.set_background_color(color);
        self
    }

    pub fn set_background_color(&mut self, color: P::Color) -> &mut Self {
        self.background_properties.set_background_color(color);
        self
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }

    pub fn bind<D>(self, data: D) -> Wrapper<Self, D>
    where
        D: WidgetData,
    {
        Wrapper::wrap(self, data)
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
        self.set_parent(parent);
        self.inner.attach(self_index, self_index + 1);
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

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We just relay whatever the child desires
        self.inner.test_input(event).map(|i| i + 1)
    }
}

impl<W, P> UpdateHandler for Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
    fn update(&mut self) {}
}

impl<W, P> ParentHolder for Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}

impl<W, P> WidgetStateHolder for Background<W, P>
where
    W: Widget,
    P: BackgroundProperties,
{
    fn change_state(&mut self, state: u32) {
        // propagate state to child widget
        self.inner.change_state(state);
        if self.state.change_state(state) {
            (self.on_state_changed)(self, self.state);
        }
    }

    fn change_selection(&mut self, _state: bool) {}

    fn is_selectable(&self) -> bool {
        false
    }
}
