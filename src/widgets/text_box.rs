use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    input::{
        controller::InputContext,
        event::{InputEvent, Key, KeyEvent, Modifier, PointerEvent},
    },
    prelude::WidgetData,
    state::{
        selection::{Selected, Unselected},
        State, WidgetState,
    },
    state_group,
    widgets::{Widget, WidgetDataHolder},
};
use heapless::String;

pub trait TextBoxProperties {
    fn measure_text(&self, text: &str, spec: MeasureSpec) -> MeasuredSize;
    fn handle_keypress<const N: usize>(
        &mut self,
        key: Key,
        modifier: Modifier,
        text: &mut String<N>,
    ) -> bool;
    fn handle_cursor_down(&mut self, coordinates: Position);
}

pub struct TextBoxFields<P, D, const N: usize> {
    pub text: String<N>,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub state: WidgetState,
    pub on_text_changed: fn(&mut D, &str),
}

impl<P, D, const N: usize> TextBoxFields<P, D, N> {
    pub fn set_text(&mut self, text: &str) -> bool {
        if self.text == text {
            return false;
        }
        self.text = String::from(text);
        true
    }
}

pub struct TextBox<P, D, const N: usize>
where
    D: WidgetData,
{
    pub fields: TextBoxFields<P, D::Data, N>,
    pub data_holder: WidgetDataHolder<TextBoxFields<P, D::Data, N>, D>,
}

impl<P, const N: usize> TextBox<P, (), N>
where
    P: TextBoxProperties,
{
    pub fn bind<D>(self, data: D) -> TextBox<P, D, N>
    where
        D: WidgetData,
    {
        TextBox {
            fields: TextBoxFields {
                parent_index: self.fields.parent_index,
                text: self.fields.text,
                bounds: self.fields.bounds,
                label_properties: self.fields.label_properties,
                state: self.fields.state,
                on_text_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<P, D, const N: usize> TextBox<P, D, N>
where
    D: WidgetData,
    P: TextBoxProperties,
{
    fn change_state(&mut self, state: impl State) -> &mut Self {
        self.fields.state.set_state(state);

        self
    }

    pub fn set_active(&mut self, active: bool) {
        if active {
            self.change_state(TextBox::STATE_ACTIVE);
        } else {
            self.change_state(TextBox::STATE_INACTIVE);
        }
    }

    pub fn on_text_changed(mut self, callback: fn(&mut D::Data, &str)) -> Self
    where
        D: WidgetData,
    {
        self.fields.on_text_changed = callback;
        self
    }

    pub fn on_data_changed(
        mut self,
        callback: fn(&mut TextBoxFields<P, D::Data, N>, &D::Data),
    ) -> Self
    where
        D: WidgetData,
    {
        self.data_holder.on_data_changed = callback;
        self
    }

    fn fire_text_changed(&mut self) {
        let callback = self.fields.on_text_changed;
        self.data_holder
            .data
            .update(|data| callback(data, &self.fields.text));
    }

    pub fn set_text(&mut self, text: &str) {
        if self.fields.set_text(text) {
            self.fire_text_changed();
        }
    }
}

state_group! {
    [TextBoxInactiveStateGroup: 0x0000_0004] = {
        Active = 0,
        Inactive = 0x0000_0004,
    }
}

impl TextBox<(), (), 0> {
    pub const STATE_INACTIVE: Inactive = Inactive;
    pub const STATE_ACTIVE: Active = Active;
    pub const STATE_SELECTED: Selected = Selected;
    pub const STATE_UNSELECTED: Unselected = Unselected;
}

impl<P, D, const N: usize> Widget for TextBox<P, D, N>
where
    D: WidgetData,
    P: TextBoxProperties,
{
    fn bounding_box(&self) -> BoundingBox {
        self.fields.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.fields.bounds
    }

    fn parent_index(&self) -> usize {
        self.fields.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.fields.parent_index = index;
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self
            .fields
            .label_properties
            .measure_text(self.fields.text.as_ref(), measure_spec);

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.fields.bounds.size = MeasuredSize { width, height };
    }

    fn on_state_changed(&mut self, _state: WidgetState) {
        // don't react to parent's state change
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.fields);
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        if self.fields.state.has_state(TextBox::STATE_INACTIVE) {
            return None;
        }

        match event {
            InputEvent::Cancel => {
                self.change_state(TextBox::STATE_UNSELECTED);
                None
            }

            InputEvent::PointerEvent(position, PointerEvent::Down) => {
                if self.bounding_box().contains(position) {
                    Some(0)
                } else {
                    // Allow a potentially clicked widget to handle the event.
                    self.change_state(TextBox::STATE_UNSELECTED);
                    None
                }
            }

            InputEvent::PointerEvent(_, PointerEvent::Drag)
            | InputEvent::PointerEvent(_, PointerEvent::Up)
            | InputEvent::PointerEvent(_, PointerEvent::Hover) => None,
            InputEvent::KeyEvent(_) => {
                if self.fields.state.has_state(TextBox::STATE_SELECTED) {
                    Some(0)
                } else {
                    None
                }
            }
            InputEvent::ScrollEvent(_) => {
                // TODO
                None
            }
        }
    }

    fn handle_input(&mut self, _ctxt: InputContext, event: InputEvent) -> bool {
        if self.fields.state.has_state(TextBox::STATE_INACTIVE) {
            return false;
        }

        match event {
            InputEvent::Cancel => {
                self.change_state(TextBox::STATE_UNSELECTED);
                true
            }
            InputEvent::PointerEvent(pos, pe) => match pe {
                // TODO: later we might want to handle drag and up to support text selection
                PointerEvent::Down => {
                    self.change_state(TextBox::STATE_SELECTED);
                    self.fields.label_properties.handle_cursor_down(pos);

                    true
                }
                _ => false,
            },
            InputEvent::KeyEvent(KeyEvent::KeyDown(keycode, modifier, _repetition_counter)) => {
                if self.fields.label_properties.handle_keypress(
                    keycode,
                    modifier,
                    &mut self.fields.text,
                ) {
                    self.fire_text_changed();
                }
                true
            }
            _ => {
                // TODO
                false
            }
        }
    }

    fn is_selectable(&self) -> bool {
        true
    }
}
