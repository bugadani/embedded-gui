use crate::{
    data::{NoData, WidgetData},
    input::{InputEvent, Key},
    widgets::{Widget, WidgetDataHolder, WidgetStateHolder, WidgetWrapper},
    BoundingBox, InputCtxt, MeasureSpec, Position, WidgetState,
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
        WidgetWrapper {
            widget: Self {
                inner,
                on_clicked: |_| (),
            },
            data_holder: WidgetDataHolder::default(),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
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
    pub fn on_clicked(mut self, callback: fn(&mut D)) -> Self
    where
        D: WidgetData,
    {
        self.on_clicked = callback;
        self
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
            widget: self.widget.bind::<D>(),
            data_holder: self.data_holder.bind(data),
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
    pub fn on_clicked(self, callback: fn(&mut D)) -> Self
    where
        Self: Sized,
    {
        WidgetWrapper {
            widget: self.widget.on_clicked(callback),
            data_holder: self.data_holder,
            on_state_changed: self.on_state_changed,
            state: self.state,
        }
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

    fn handle_input(&mut self, ctxt: &mut InputCtxt, event: InputEvent) -> bool {
        match event {
            InputEvent::KeyUp(Key::Space, _) => self.fire_on_clicked(),
            InputEvent::KeyUp(Key::Tab, _) => ctxt.select_next_widget(),
            InputEvent::PointerHover(pos) => {
                if self.bounding_box().hit_test(pos) {
                    self.change_state(Button::STATE_HOVERED);
                } else {
                    self.change_state(Button::STATE_IDLE);
                }
            }
            InputEvent::PointerDown(pos) => {
                if self.bounding_box().hit_test(pos) {
                    self.change_state(Button::STATE_PRESSED);
                    self.fire_on_pressed();
                } else {
                    self.change_state(Button::STATE_IDLE);
                }
            }
            InputEvent::PointerMove(pos) => {
                if !self.bounding_box().hit_test(pos) {
                    self.change_state(Button::STATE_IDLE);
                }
            }
            InputEvent::PointerUp(pos) => {
                if self.bounding_box().hit_test(pos) {
                    self.change_state(Button::STATE_HOVERED);
                    self.fire_on_clicked();
                } else {
                    self.change_state(Button::STATE_IDLE);
                }
            }
            _ => return false,
        }

        true
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}
