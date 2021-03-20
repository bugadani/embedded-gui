use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    input::event::InputEvent,
    widgets::{ParentHolder, Widget, WidgetDataHolder, WidgetStateHolder, WidgetWrapper},
    BoundingBox, MeasureSpec, Position, WidgetState,
};

pub trait BackgroundProperties {
    type Color;

    fn background_color(&mut self, color: Self::Color);
}

pub struct Background<I, P, D>
where
    P: BackgroundProperties,
    D: WidgetData,
{
    pub inner: I,
    pub background_properties: P,
    pub _marker: PhantomData<D>,
}

impl<I, P> Background<I, P, NoData>
where
    I: Widget,
    P: BackgroundProperties + Default,
{
    pub fn new(inner: I) -> WidgetWrapper<Background<I, P, NoData>, NoData> {
        WidgetWrapper::new(Background {
            background_properties: P::default(),
            inner,
            _marker: PhantomData,
        })
    }
}

impl<W, P> Background<W, P, NoData>
where
    W: Widget,
    P: BackgroundProperties,
{
    pub fn bind<D>(self) -> Background<W, P, D>
    where
        D: WidgetData,
    {
        Background {
            inner: self.inner,
            background_properties: self.background_properties,
            _marker: PhantomData,
        }
    }
}

impl<W, P, D> Background<W, P, D>
where
    W: Widget,
    P: BackgroundProperties,
    D: WidgetData,
{
    pub fn background_color(&mut self, color: P::Color) {
        self.background_properties.background_color(color);
    }
}

impl<W, P> WidgetWrapper<Background<W, P, NoData>, NoData>
where
    W: Widget,
    P: BackgroundProperties,
{
    pub fn bind<D>(self, data: D) -> WidgetWrapper<Background<W, P, D>, D>
    where
        D: WidgetData,
    {
        WidgetWrapper {
            parent_index: self.parent_index,
            widget: self.widget.bind::<D>(),
            data_holder: WidgetDataHolder::<Background<W, P, D>, NoData>::default().bind(data),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

impl<W, P, D> WidgetWrapper<Background<W, P, D>, D>
where
    W: Widget,
    P: BackgroundProperties,
    D: WidgetData,
{
    pub fn background_color(mut self, color: P::Color) -> Self {
        self.widget.background_color(color);
        self
    }
}

impl<W, P, D> WidgetStateHolder for WidgetWrapper<Background<W, P, D>, D>
where
    W: Widget,
    P: BackgroundProperties,
    D: WidgetData,
{
    fn change_state(&mut self, state: u32) {
        // propagate state to child widget
        self.widget.inner.change_state(state);
        if self.state.change_state(state) {
            (self.on_state_changed)(&mut self.widget, self.state)
        }
    }

    fn change_selection(&mut self, _state: bool) {}

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<W, P, D> Widget for WidgetWrapper<Background<W, P, D>, D>
where
    W: Widget,
    P: BackgroundProperties,
    D: WidgetData,
{
    fn attach(&mut self, parent: Option<usize>, self_index: usize) {
        self.set_parent(parent);
        self.widget.inner.attach(Some(self_index), self_index + 1);
    }

    fn arrange(&mut self, position: Position) {
        self.widget.inner.arrange(position);
    }

    fn bounding_box(&self) -> BoundingBox {
        self.widget.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.widget.inner.measure(measure_spec);
    }

    fn children(&self) -> usize {
        1 + self.widget.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.widget.inner
        } else {
            self.widget.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.widget.inner
        } else {
            self.widget.inner.get_mut_child(idx - 1)
        }
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We just relay whatever the child desires
        self.widget.inner.test_input(event).map(|i| i + 1)
    }
}
