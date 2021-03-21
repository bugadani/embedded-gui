use crate::{input::event::InputEvent, widgets::Widget};

pub trait InputController {
    fn input_event(&mut self, root: &mut impl Widget, event: InputEvent);
}

pub struct InputContext {}

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
        let handler = if let Some(last) = self.last_handler {
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

        if let Some(mut handler) = handler {
            let actual_handler = loop {
                let widget = self.get_mut_widget(root, handler);
                let context = InputContext {};
                if widget.handle_input(context, event) {
                    break Some(handler);
                } else {
                    let parent = widget.parent_index();
                    if parent == 0 && handler == 0 {
                        // I am Root
                        break None;
                    } else {
                        handler = parent;
                    }
                }
            };

            self.last_handler = actual_handler;
        }
    }
}
