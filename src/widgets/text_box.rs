use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    input::{
        controller::InputContext,
        event::{InputEvent, KeyEvent, PointerEvent},
    },
    state::{
        selection::{Selected, Unselected},
        State, WidgetState,
    },
    state_group,
    widgets::{wrapper::WrapperBindable, Widget},
};

pub trait TextBoxProperties {
    fn measure_text(&self, text: &str, spec: MeasureSpec) -> MeasuredSize;
}

pub struct TextBox<S, P> {
    pub text: S,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub state: WidgetState,
}

impl<S, P> TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
    fn change_state(&mut self, state: impl State) -> &mut Self {
        self.state.set_state(state);

        self
    }

    pub fn set_active(&mut self, active: bool) {
        if active {
            self.change_state(TextBox::STATE_ACTIVE);
        } else {
            self.change_state(TextBox::STATE_INACTIVE);
        }
    }
}

state_group! {
    [TextBoxInactiveStateGroup: 0x0000_0004] = {
        Active = 0,
        Inactive = 0x0000_0004,
    }
}

impl TextBox<(), ()> {
    pub const STATE_INACTIVE: Inactive = Inactive;
    pub const STATE_ACTIVE: Active = Active;
    pub const STATE_SELECTED: Selected = Selected;
    pub const STATE_UNSELECTED: Unselected = Unselected;
}

impl<S, P> Widget for TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self
            .label_properties
            .measure_text(self.text.as_ref(), measure_spec);

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.bounds.size = MeasuredSize { width, height };
    }

    fn on_state_changed(&mut self, _state: WidgetState) {
        // don't react to parent's state change
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        if self.state.has_state(TextBox::STATE_INACTIVE) {
            return None;
        }

        match event {
            InputEvent::Cancel => {
                self.change_state(TextBox::STATE_UNSELECTED);
                None
            }

            InputEvent::PointerEvent(position, PointerEvent::Down) => {
                if self.bounding_box().contains(position)
                    || self.state.has_state(TextBox::STATE_SELECTED)
                {
                    Some(0)
                } else {
                    None
                }
            }

            InputEvent::PointerEvent(_, PointerEvent::Drag)
            | InputEvent::PointerEvent(_, PointerEvent::Up)
            | InputEvent::PointerEvent(_, PointerEvent::Hover) => None,
            InputEvent::KeyEvent(_) => {
                if self.state.has_state(TextBox::STATE_SELECTED) {
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
        if self.state.has_state(TextBox::STATE_INACTIVE) {
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
                    if self.bounding_box().contains(pos) {
                        self.change_state(TextBox::STATE_SELECTED);
                        // TODO send to TextBox impl
                    } else {
                        self.change_state(TextBox::STATE_UNSELECTED);
                    }

                    true
                }
                _ => false,
            },
            InputEvent::KeyEvent(KeyEvent::KeyDown(keycode, modifier, _repetition_counter)) => {
                // TODO send to TextBox impl
                println!("{:?}", keycode);
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

impl<S, P> WrapperBindable for TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
}
