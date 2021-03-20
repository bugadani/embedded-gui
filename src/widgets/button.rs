use crate::{
    data::{NoData, WidgetData},
    input::event::InputEvent,
    widgets::{ParentHolder, Widget, WidgetDataHolder, WidgetStateHolder, WidgetWrapper},
    BoundingBox, MeasureSpec, Position, WidgetState,
};

pub struct Button<I, D>
where
    D: WidgetData,
{
    pub inner: I,
    pub on_clicked: fn(&mut D),
}

impl Button<(), NoData> {
    pub const STATE_IDLE: u32 = 0;
    pub const STATE_HOVERED: u32 = 1;
    pub const STATE_PRESSED: u32 = 2;
}

impl<I> Button<I, NoData>
where
    I: Widget,
{
    pub fn new(inner: I) -> WidgetWrapper<Self, NoData> {
        WidgetWrapper::new(Button {
            inner,
            on_clicked: |_| (),
        })
    }

    pub fn bind<D>(self) -> Button<I, D>
    where
        D: WidgetData,
    {
        Button {
            inner: self.inner,
            on_clicked: |_| (),
        }
    }
}

impl<I, D> Button<I, D>
where
    I: Widget,
    D: WidgetData,
{
    pub fn on_clicked(&mut self, callback: fn(&mut D))
    where
        D: WidgetData,
    {
        self.on_clicked = callback;
    }
}

impl<I> WidgetWrapper<Button<I, NoData>, NoData>
where
    I: Widget,
{
    pub fn bind<D>(self, data: D) -> WidgetWrapper<Button<I, D>, D>
    where
        D: WidgetData,
    {
        WidgetWrapper {
            parent_index: self.parent_index,
            widget: self.widget.bind::<D>(),
            data_holder: WidgetDataHolder::<Button<I, D>, NoData>::default().bind(data),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

impl<I, D> WidgetWrapper<Button<I, D>, D>
where
    I: Widget,
    D: WidgetData,
{
    pub fn on_clicked(mut self, callback: fn(&mut D)) -> Self
    where
        Self: Sized,
    {
        self.apply(|widget| widget.on_clicked(callback));
        self
    }

    fn fire_on_pressed(&mut self) {}
    fn fire_on_clicked(&mut self) {
        let callback = self.widget.on_clicked;
        callback(&mut self.data_holder.data)
    }
}

impl<I, D> WidgetStateHolder for WidgetWrapper<Button<I, D>, D>
where
    I: Widget,
    D: WidgetData,
{
    fn change_state(&mut self, state: u32) {
        // propagate state to child widget
        self.widget.inner.change_state(state);
        // apply state
        if self.state.change_state(state) {
            (self.on_state_changed)(&mut self.widget, self.state);
        }
    }

    fn change_selection(&mut self, state: bool) {
        // apply state
        if self.state.change_selection(state) {
            (self.on_state_changed)(&mut self.widget, self.state);
        }
        if !state {
            // while this isn't correct (i.e. deselecting by keyboard removes hover state)
            // it might be good enough
            self.change_state(Button::STATE_IDLE);
        }
    }

    fn is_selectable(&self) -> bool {
        true
    }
}

impl<I, D> Widget for WidgetWrapper<Button<I, D>, D>
where
    I: Widget,
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
        self.widget.inner.measure(measure_spec)
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

    fn handle_input(&mut self, _event: InputEvent) -> bool {
        // TODO
        true
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}
