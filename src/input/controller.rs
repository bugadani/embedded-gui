use crate::{input::event::InputEvent, widgets::Widget};

pub trait InputController {
    fn input_event(&mut self, root: &mut impl Widget, event: InputEvent);
}

pub struct DefaultInputController {}

impl DefaultInputController {
    pub fn new() -> Self {
        Self {}
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
        if let Some(handler) = root.test_input(event) {
            self.get_mut_widget(root, handler).handle_input(event);
        }
    }
}
