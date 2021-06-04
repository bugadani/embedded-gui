use crate::{input::event::InputEvent, widgets::Widget};

pub trait InputController {
    fn input_event(&mut self, root: &mut impl Widget, event: InputEvent);
}

pub struct InputContext {
    bubbled: bool,
}

impl InputContext {
    /// Returns whether the input event is bubbled.
    pub fn is_bubbled(&self) -> bool {
        self.bubbled
    }
}

pub struct DefaultInputController {
    last_handler: Option<usize>,
}

impl DefaultInputController {
    pub fn new() -> Self {
        Self { last_handler: None }
    }

    fn get_mut_widget<'a>(
        &'a mut self,
        root: &'a mut impl Widget,
        idx: usize,
    ) -> &'a mut dyn Widget {
        if idx == 0 {
            &mut *root
        } else {
            root.get_mut_child(idx - 1)
        }
    }
}

impl InputController for DefaultInputController {
    fn input_event(&mut self, root: &mut impl Widget, event: InputEvent) {
        self.last_handler = if let Some(last) = self.last_handler {
            if let Some(handler) = self.get_mut_widget(root, last).test_input(event) {
                // it's possible the widget wants to pass the event to it's child
                Some(last + handler)
            } else {
                // the widget doesn't want to handle the event any more, find a new target
                root.test_input(event)
            }
        } else {
            root.test_input(event)
        };

        if let Some(orig_handler) = self.last_handler {
            let mut handler = orig_handler;
            loop {
                let widget = self.get_mut_widget(root, handler);
                let context = InputContext { bubbled: false };
                if widget.handle_input(context, event) {
                    if handler != orig_handler {
                        // parent handled a bubbled event, should notify child somehow
                        let widget = self.get_mut_widget(root, orig_handler);
                        let context = InputContext { bubbled: true };
                        widget.handle_input(context, InputEvent::Cancel);
                    }
                    break;
                } else {
                    let parent = widget.parent_index();
                    if parent == 0 && handler == 0 {
                        // I am Root
                        break;
                    } else {
                        debug_assert!(parent != handler);
                        handler = parent;
                    }
                }
            }
        }
    }
}
