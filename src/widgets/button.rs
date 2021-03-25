use crate::{
    data::{NoData, WidgetData},
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent},
    },
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
    BoundingBox, Canvas, MeasureSpec, Position, WidgetRenderer, WidgetState,
};

pub struct Button<W, D = NoData>
where
    D: WidgetData,
{
    pub parent_index: usize,
    pub inner: W,
    pub on_clicked: fn(&mut D::Data),

    // FIXME: Borrow checker freaks out because update needs both widget and data.
    // Should avoid possible runtime overhead, though.
    data_holder: Option<WidgetDataHolder<Self, D>>,

    pub on_state_changed: fn(&mut Self, WidgetState),
    pub state: WidgetState,
}

impl Button<(), NoData> {
    pub const STATE_IDLE: u32 = 0;
    pub const STATE_HOVERED: u32 = 1;
    pub const STATE_PRESSED: u32 = 2;
}

impl<W> Button<W, NoData>
where
    W: Widget,
{
    pub fn new(inner: W) -> Self {
        Button {
            parent_index: 0,
            inner,
            on_clicked: |_| (),
            data_holder: Some(WidgetDataHolder::default()),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }

    pub fn bind<D>(self, data: D) -> Button<W, D>
    where
        D: WidgetData,
    {
        Button {
            parent_index: self.parent_index,
            inner: self.inner,
            on_clicked: |_| (),
            data_holder: Some(WidgetDataHolder::new(data)),
            on_state_changed: |_, _| (),
            state: self.state,
        }
    }
}

impl<W, D> Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn on_clicked(mut self, callback: fn(&mut D::Data)) -> Self
    where
        D: WidgetData,
    {
        self.on_clicked = callback;
        self
    }

    fn fire_on_pressed(&mut self) {}
    fn fire_on_clicked(&mut self) {
        let callback = self.on_clicked;
        self.data_holder
            .as_mut()
            .map(|holder| holder.data.update(callback));
    }
}

impl<W, D> WidgetStateHolder for Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn change_state(&mut self, state: u32) {
        // propagate state to child widget
        self.inner.change_state(state);
        // apply state
        if self.state.change_state(state) {
            (self.on_state_changed)(self, self.state);
        }
    }

    fn change_selection(&mut self, state: bool) {
        // apply state
        if self.state.change_selection(state) {
            (self.on_state_changed)(self, self.state);
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

impl<W, D> Widget for Button<W, D>
where
    W: Widget,
    D: WidgetData,
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
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec)
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
        match event {
            InputEvent::PointerEvent(position, PointerEvent::Down) => {
                if let Some(idx) = self.inner.test_input(event) {
                    // we give priority to our child
                    Some(idx + 1)
                } else if self.bounding_box().contains(position) {
                    Some(0)
                } else {
                    None
                }
            }

            InputEvent::PointerEvent(position, PointerEvent::Drag) => {
                if self.bounding_box().contains(position) {
                    if self.state.state() != Button::STATE_PRESSED {
                        self.change_state(Button::STATE_HOVERED);
                    }
                    Some(0)
                } else {
                    // Drag outside = cancel
                    self.change_state(Button::STATE_IDLE);
                    None
                }
            }

            // We only get Up if we handled Down
            InputEvent::PointerEvent(_, PointerEvent::Up) => {
                if self.state.state() == Button::STATE_PRESSED {
                    Some(0)
                } else {
                    None
                }
            }

            InputEvent::PointerEvent(position, PointerEvent::Hover) => {
                if let Some(idx) = self.inner.test_input(event) {
                    // we give priority to our child
                    self.change_state(Button::STATE_IDLE);
                    Some(idx + 1)
                } else if self.bounding_box().contains(position) {
                    Some(0)
                } else {
                    // Make sure we reset our state if we don't handle the pointer event.
                    // It's possible we were the target for the last one.
                    self.change_state(Button::STATE_IDLE);
                    None
                }
            }
            InputEvent::KeyEvent(_) => {
                // TODO we do care about some key events
                None
            }
            InputEvent::ScrollEvent(_) => None,
        }
    }

    fn handle_input(&mut self, _ctxt: InputContext, event: InputEvent) -> bool {
        match event {
            InputEvent::PointerEvent(_, pe) => match pe {
                PointerEvent::Hover => self.change_state(Button::STATE_HOVERED),
                PointerEvent::Down => {
                    self.fire_on_pressed();
                    self.change_state(Button::STATE_PRESSED);
                }
                PointerEvent::Drag => {}
                PointerEvent::Up => {
                    self.change_state(Button::STATE_HOVERED);
                    self.fire_on_clicked();
                }
            },
            _ => {
                // TODO
            }
        }
        true
    }
}

impl<W, D> UpdateHandler for Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn update(&mut self) {
        let mut data = self.data_holder.take().unwrap();
        data.update(self);
        self.data_holder = Some(data);
    }
}

impl<W, D> ParentHolder for Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}

impl<C, W, D> WidgetRenderer<C> for Button<W, D>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.inner.draw(canvas)
    }
}
