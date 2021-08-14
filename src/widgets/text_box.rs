//! Text box widget to display editable text

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
use core::borrow::BorrowMut;
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

pub struct TextBoxFields<B, P, D, const N: usize>
where
    B: BorrowMut<String<N>>,
{
    pub text: B,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub state: WidgetState,
    pub on_text_changed: fn(&mut D, &str),
    pub on_parent_state_changed: fn(&mut Self, WidgetState),
}

impl<B, P, D, const N: usize> TextBoxFields<B, P, D, N>
where
    B: BorrowMut<String<N>>,
{
    pub fn set_text(&mut self, text: &str) -> bool {
        if self.text.borrow() == text {
            return false;
        }
        *self.text.borrow_mut() = String::from(text);
        true
    }
}

pub struct TextBox<B, P, D, const N: usize>
where
    B: BorrowMut<String<N>>,
    D: WidgetData,
{
    pub fields: TextBoxFields<B, P, D::Data, N>,
    pub data_holder: WidgetDataHolder<TextBoxFields<B, P, D::Data, N>, D>,
}

impl<B, P, const N: usize> TextBox<B, P, (), N>
where
    B: BorrowMut<String<N>>,
    P: TextBoxProperties,
{
    pub fn bind<D>(self, data: D) -> TextBox<B, P, D, N>
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
                on_parent_state_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<B, P, D, const N: usize> TextBox<B, P, D, N>
where
    B: BorrowMut<String<N>>,
    D: WidgetData,
    P: TextBoxProperties,
{
    pub fn on_state_changed(
        mut self,
        callback: fn(&mut TextBoxFields<B, P, D::Data, N>, WidgetState),
    ) -> Self {
        self.fields.on_parent_state_changed = callback;
        self
    }

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
        callback: fn(&mut TextBoxFields<B, P, D::Data, N>, &D::Data),
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
            .update(|data| callback(data, self.fields.text.borrow()));
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

impl TextBox<String<0>, (), (), 0> {
    pub const STATE_INACTIVE: Inactive = Inactive;
    pub const STATE_ACTIVE: Active = Active;
    pub const STATE_SELECTED: Selected = Selected;
    pub const STATE_UNSELECTED: Unselected = Unselected;
}

impl<B, P, D, const N: usize> Widget for TextBox<B, P, D, N>
where
    B: BorrowMut<String<N>>,
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
            .measure_text(self.fields.text.borrow(), measure_spec);

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.fields.bounds.size = MeasuredSize { width, height };
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        (self.fields.on_parent_state_changed)(&mut self.fields, state);
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
                    self.fields.text.borrow_mut(),
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
