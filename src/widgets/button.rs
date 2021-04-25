use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, Position},
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent},
    },
    state::{State, StateGroup, WidgetState},
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
    Canvas, WidgetRenderer,
};

// It's necessary to split off the non-data fields so that callbacks can work on the widget while
// the data is borrowed.
pub struct ButtonFields<W, D> {
    pub parent_index: usize,
    pub inner: W,
    pub on_clicked: fn(&mut D),
    pub state: WidgetState,
}

impl<W, D> ButtonFields<W, D>
where
    W: Widget,
{
    fn change_state(&mut self, state: impl State) -> &mut Self {
        if self.state.set_state(state) {
            self.inner.on_state_changed(self.state);
        }

        self
    }

    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        if enabled {
            self.change_state(Button::STATE_ENABLED);
        } else {
            self.change_state(Button::STATE_DISABLED);
        }

        self
    }
}

pub struct Button<W, D = ()>
where
    D: WidgetData,
{
    pub fields: ButtonFields<W, D::Data>,
    data_holder: WidgetDataHolder<ButtonFields<W, D::Data>, D>,
}

pub struct ButtonStateGroup;
impl StateGroup for ButtonStateGroup {
    const MASK: u32 = 0x0000_0003;
}

pub struct Idle;
impl State for Idle {
    type Group = ButtonStateGroup;

    const VALUE: u32 = 0x0000_0000;
}

pub struct Hovered;
impl State for Hovered {
    type Group = ButtonStateGroup;

    const VALUE: u32 = 0x0000_0001;
}

pub struct Pressed;
impl State for Pressed {
    type Group = ButtonStateGroup;

    const VALUE: u32 = 0x0000_0002;
}

pub struct ButtonDisabledStateGroup;
impl StateGroup for ButtonDisabledStateGroup {
    const MASK: u32 = 0x0000_0004;
}

pub struct Enabled;
impl State for Enabled {
    type Group = ButtonDisabledStateGroup;

    const VALUE: u32 = 0x0000_0000;
}

pub struct Disabled;
impl State for Disabled {
    type Group = ButtonDisabledStateGroup;

    const VALUE: u32 = 0x0000_0004;
}

impl Button<(), ()> {
    pub const STATE_IDLE: Idle = Idle;
    pub const STATE_HOVERED: Hovered = Hovered;
    pub const STATE_PRESSED: Pressed = Pressed;
    pub const STATE_DISABLED: Disabled = Disabled;
    pub const STATE_ENABLED: Enabled = Enabled;
}

impl<W> Button<W, ()>
where
    W: Widget,
{
    pub fn new(inner: W) -> Self {
        Button {
            fields: ButtonFields {
                parent_index: 0,
                inner,
                on_clicked: |_| (),
                state: WidgetState::default(),
            },
            data_holder: WidgetDataHolder::default(),
        }
    }

    pub fn bind<D>(self, data: D) -> Button<W, D>
    where
        D: WidgetData,
    {
        Button {
            fields: ButtonFields {
                parent_index: self.fields.parent_index,
                inner: self.fields.inner,
                on_clicked: |_| (),
                state: self.fields.state,
            },
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<W, D> Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.set_enabled(enabled);
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.fields.set_enabled(enabled);
        self
    }

    pub fn on_data_changed(mut self, callback: fn(&mut ButtonFields<W, D::Data>, &D::Data)) -> Self
    where
        D: WidgetData,
    {
        self.data_holder.on_data_changed = callback;
        self
    }

    pub fn on_clicked(mut self, callback: fn(&mut D::Data)) -> Self
    where
        D: WidgetData,
    {
        self.fields.on_clicked = callback;
        self
    }

    fn fire_on_clicked(&mut self) {
        let callback = self.fields.on_clicked;
        self.data_holder.data.update(callback);
    }
}

impl<W, D> WidgetStateHolder for Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn on_state_changed(&mut self, _state: WidgetState) {
        // don't react to parent's state change
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
        self.fields.inner.attach(self_index, self_index + 1);
    }

    fn arrange(&mut self, position: Position) {
        self.fields.inner.arrange(position);
    }

    fn bounding_box(&self) -> BoundingBox {
        self.fields.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.fields.inner.measure(measure_spec)
    }

    fn children(&self) -> usize {
        1 + self.fields.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.fields.inner
        } else {
            self.fields.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.fields.inner
        } else {
            self.fields.inner.get_mut_child(idx - 1)
        }
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        if self.fields.state.has_state(Button::STATE_DISABLED) {
            return None;
        }

        match event {
            InputEvent::Cancel => {
                self.fields.change_state(Button::STATE_IDLE);
                None
            }

            InputEvent::PointerEvent(position, PointerEvent::Down) => {
                if let Some(idx) = self.fields.inner.test_input(event) {
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
                    if !self.fields.state.has_state(Button::STATE_PRESSED) {
                        self.fields.change_state(Button::STATE_HOVERED);
                    }
                    Some(0)
                } else {
                    // Drag outside = cancel
                    self.fields.change_state(Button::STATE_IDLE);
                    None
                }
            }

            // We only get Up if we handled Down
            InputEvent::PointerEvent(_, PointerEvent::Up) => {
                if self.fields.state.has_state(Button::STATE_PRESSED) {
                    Some(0)
                } else {
                    None
                }
            }

            InputEvent::PointerEvent(position, PointerEvent::Hover) => {
                if let Some(idx) = self.fields.inner.test_input(event) {
                    // we give priority to our child
                    self.fields.change_state(Button::STATE_IDLE);
                    Some(idx + 1)
                } else if self.bounding_box().contains(position) {
                    self.fields.change_state(Button::STATE_HOVERED);
                    // We deliberately don't handle hover events. In case the button is partially
                    // displayed, handling hover would route clicks that fall on the hidden parts.
                    None
                } else {
                    // Make sure we reset our state if we don't handle the pointer event.
                    // It's possible we were the target for the last one.
                    self.fields.change_state(Button::STATE_IDLE);
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
        if self.fields.state.has_state(Button::STATE_DISABLED) {
            return false;
        }

        match event {
            InputEvent::Cancel => {
                if self.fields.state.has_state(Button::STATE_PRESSED) {
                    self.fields.change_state(Button::STATE_HOVERED);
                }
                true
            }
            InputEvent::PointerEvent(_, pe) => match pe {
                PointerEvent::Hover | PointerEvent::Drag => false,
                PointerEvent::Down => {
                    self.fields.change_state(Button::STATE_PRESSED);
                    true
                }
                PointerEvent::Up => {
                    self.fields.change_state(Button::STATE_HOVERED);
                    self.fire_on_clicked();
                    true
                }
            },
            _ => {
                // TODO
                false
            }
        }
    }
}

impl<W, D> UpdateHandler for Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.fields);
    }
}

impl<W, D> ParentHolder for Button<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn parent_index(&self) -> usize {
        self.fields.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.fields.parent_index = index;
    }
}

impl<C, W, D> WidgetRenderer<C> for Button<W, D>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.fields.inner.draw(canvas)
    }
}
