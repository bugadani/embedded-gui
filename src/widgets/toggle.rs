use crate::{
    data::WidgetData,
    geometry::BoundingBox,
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent},
    },
    state::{State, StateGroup, WidgetState},
    state_group,
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
    Canvas, MeasureSpec, Position, WidgetRenderer,
};

// It's necessary to split off the non-data fields so that callbacks can work on the widget while
// the data is borrowed.
pub struct ToggleFields<W, D> {
    pub parent_index: usize,
    pub inner: W,
    pub on_selected_changed: fn(bool, &mut D),
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub state: WidgetState,
}

impl<W, D> ToggleFields<W, D>
where
    W: Widget,
{
    fn change_state(&mut self, state: impl State) -> &mut Self {
        if self.state.set_state(state) {
            self.inner.on_state_changed(self.state);
        }

        self
    }

    pub fn set_active(&mut self, active: bool) -> &mut Self {
        if active {
            self.change_state(Toggle::STATE_ACTIVE);
        } else {
            self.change_state(Toggle::STATE_INACTIVE);
        }

        self
    }

    pub fn is_checked(&self) -> bool {
        self.state.has_state(Toggle::STATE_CHECKED)
    }

    pub fn set_checked(&mut self, checked: bool) -> &mut Self {
        if checked {
            self.change_state(Toggle::STATE_CHECKED);
        } else {
            self.change_state(Toggle::STATE_UNCHECKED);
        }

        self
    }
}

pub struct Toggle<W, D, const MANUAL_UNCHECK: bool>
where
    D: WidgetData,
{
    pub fields: ToggleFields<W, D::Data>,
    data_holder: WidgetDataHolder<ToggleFields<W, D::Data>, D>,
}

state_group! {
    [ToggleStateGroup: 0x0000_0003] = {
        Idle = 0,
        Hovered = 0x0000_0001,
        Pressed = 0x0000_0002,
    }
    [ToggleCheckedStateGroup: 0x0000_0004] = {
        Unchecked = 0,
        Checked = 0x0000_0004,
    }
    [ToggleInactiveStateGroup: 0x0000_0008] = {
        Active = 0,
        Inactive = 0x0000_0008
    }
}

impl Toggle<(), (), true> {
    pub const STATE_IDLE: Idle = Idle;
    pub const STATE_HOVERED: Hovered = Hovered;
    pub const STATE_PRESSED: Pressed = Pressed;
    pub const STATE_INACTIVE: Inactive = Inactive;
    pub const STATE_ACTIVE: Active = Active;

    pub const STATE_CHECKED: Checked = Checked;
    pub const STATE_UNCHECKED: Unchecked = Unchecked;
}

impl<W> Toggle<W, (), true>
where
    W: Widget,
{
    pub fn new(inner: W) -> Self {
        Toggle {
            fields: ToggleFields {
                parent_index: 0,
                inner,
                on_selected_changed: |_, _| (),
                on_state_changed: |_, _| (),
                state: WidgetState::default(),
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W, const MANUAL_UNCHECK: bool> Toggle<W, (), MANUAL_UNCHECK>
where
    W: Widget,
{
    pub fn bind<D>(self, data: D) -> Toggle<W, D, MANUAL_UNCHECK>
    where
        D: WidgetData,
    {
        Toggle {
            fields: ToggleFields {
                parent_index: self.fields.parent_index,
                inner: self.fields.inner,
                on_selected_changed: |_, _| (),
                on_state_changed: |_, _| (),
                state: self.fields.state,
            },
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<W, D> Toggle<W, D, true>
where
    W: Widget,
    D: WidgetData,
{
    pub fn disallow_manual_uncheck(self) -> Toggle<W, D, false> {
        Toggle {
            fields: self.fields,
            data_holder: self.data_holder,
        }
    }
}

impl<W, D, const MANUAL_UNCHECK: bool> Toggle<W, D, MANUAL_UNCHECK>
where
    W: Widget,
    D: WidgetData,
{
    pub fn active(mut self, active: bool) -> Self {
        self.set_active(active);
        self
    }

    pub fn set_active(&mut self, active: bool) -> &mut Self {
        self.fields.set_active(active);

        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.set_checked(checked);
        self
    }

    pub fn set_checked(&mut self, checked: bool) -> &mut Self {
        self.fields.set_checked(checked);

        self
    }

    pub fn on_data_changed(mut self, callback: fn(&mut ToggleFields<W, D::Data>, &D::Data)) -> Self
    where
        D: WidgetData,
    {
        self.data_holder.on_data_changed = callback;
        self
    }

    pub fn on_selected_changed(mut self, callback: fn(bool, &mut D::Data)) -> Self
    where
        D: WidgetData,
    {
        self.fields.on_selected_changed = callback;
        self
    }

    fn fire_on_selected_changed(&mut self) {
        let is_checked = self.fields.state.has_state(Toggle::STATE_CHECKED);
        if MANUAL_UNCHECK || !is_checked {
            self.fields.set_checked(!is_checked);

            let callback = self.fields.on_selected_changed;
            self.data_holder
                .data
                .update(|data| callback(!is_checked, data));
        }
    }
}

impl<W, D, const MANUAL_UNCHECK: bool> WidgetStateHolder for Toggle<W, D, MANUAL_UNCHECK>
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

impl<W, D, const MANUAL_UNCHECK: bool> Widget for Toggle<W, D, MANUAL_UNCHECK>
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
        if self.fields.state.has_state(Toggle::STATE_INACTIVE) {
            return None;
        }

        match event {
            InputEvent::Cancel => {
                self.fields.change_state(Toggle::STATE_IDLE);
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
                    if !self.fields.state.has_state(Toggle::STATE_PRESSED) {
                        self.fields.change_state(Toggle::STATE_HOVERED);
                    }
                    Some(0)
                } else {
                    // Drag outside = cancel
                    self.fields.change_state(Toggle::STATE_IDLE);
                    None
                }
            }

            // We only get Up if we handled Down
            InputEvent::PointerEvent(_, PointerEvent::Up) => {
                if self.fields.state.has_state(Toggle::STATE_PRESSED) {
                    Some(0)
                } else {
                    None
                }
            }

            InputEvent::PointerEvent(position, PointerEvent::Hover) => {
                if let Some(idx) = self.fields.inner.test_input(event) {
                    // we give priority to our child
                    self.fields.change_state(Toggle::STATE_IDLE);
                    Some(idx + 1)
                } else if self.bounding_box().contains(position) {
                    self.fields.change_state(Toggle::STATE_HOVERED);
                    // We deliberately don't handle hover events. In case the button is partially
                    // displayed, handling hover would route clicks that fall on the hidden parts.
                    None
                } else {
                    // Make sure we reset our state if we don't handle the pointer event.
                    // It's possible we were the target for the last one.
                    self.fields.change_state(Toggle::STATE_IDLE);
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
        if self.fields.state.has_state(Toggle::STATE_INACTIVE) {
            return false;
        }

        match event {
            InputEvent::Cancel => {
                if self.fields.state.has_state(Toggle::STATE_PRESSED) {
                    self.fields.change_state(Toggle::STATE_HOVERED);
                }
                true
            }
            InputEvent::PointerEvent(_, pe) => match pe {
                PointerEvent::Hover | PointerEvent::Drag => false,
                PointerEvent::Down => {
                    self.fields.change_state(Toggle::STATE_PRESSED);
                    true
                }
                PointerEvent::Up => {
                    self.fields.change_state(Toggle::STATE_HOVERED);
                    self.fire_on_selected_changed();
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

impl<W, D, const MANUAL_UNCHECK: bool> UpdateHandler for Toggle<W, D, MANUAL_UNCHECK>
where
    W: Widget,
    D: WidgetData,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.fields);
    }
}

impl<W, D, const MANUAL_UNCHECK: bool> ParentHolder for Toggle<W, D, MANUAL_UNCHECK>
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

impl<C, W, D, const MANUAL_UNCHECK: bool> WidgetRenderer<C> for Toggle<W, D, MANUAL_UNCHECK>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.fields.inner.draw(canvas)
    }
}
