//! Helper to simplify implementation of widget decorators.

use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    input::{controller::InputContext, event::InputEvent},
    state::WidgetState,
    widgets::Widget,
};

pub trait WidgetDecorator {
    type Widget: Widget;

    fn attach(&mut self, parent: usize, self_index: usize) {
        self.widget_mut().attach(parent, self_index);
    }

    fn bounding_box(&self) -> BoundingBox {
        self.widget().bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        self.widget_mut().bounding_box_mut()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.widget_mut().measure(measure_spec);
    }

    fn arrange(&mut self, position: Position) {
        self.widget_mut().arrange(position);
    }

    fn set_measured_size(&mut self, size: MeasuredSize) {
        self.widget_mut().set_measured_size(size);
    }

    fn children(&self) -> usize {
        1 + self.widget().children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            self.widget()
        } else {
            self.widget().get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            self.widget_mut()
        } else {
            self.widget_mut().get_mut_child(idx - 1)
        }
    }

    fn parent_index(&self) -> usize {
        self.widget().parent_index()
    }

    fn update(&mut self) {
        self.widget_mut().update();
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.fire_on_state_changed(state);
        self.widget_mut().on_state_changed(state);
    }

    fn fire_on_state_changed(&mut self, _state: WidgetState) {}

    fn set_parent(&mut self, _index: usize) {
        // TODO this doesn't belong in the Widget interface
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.widget_mut().test_input(event).map(|i| i + 1)
    }

    fn handle_input(&mut self, ctxt: InputContext, event: InputEvent) -> bool {
        self.widget_mut().handle_input(ctxt, event)
    }

    fn is_selectable(&self) -> bool {
        self.widget().is_selectable()
    }

    fn widget(&self) -> &Self::Widget;

    fn widget_mut(&mut self) -> &mut Self::Widget;
}

impl<T> Widget for T
where
    T: WidgetDecorator,
{
    fn attach(&mut self, parent: usize, self_index: usize) {
        WidgetDecorator::attach(self, parent, self_index);
    }

    fn bounding_box(&self) -> BoundingBox {
        WidgetDecorator::bounding_box(self)
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        WidgetDecorator::bounding_box_mut(self)
    }

    fn children(&self) -> usize {
        WidgetDecorator::children(self)
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        WidgetDecorator::get_child(self, idx)
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        WidgetDecorator::get_mut_child(self, idx)
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        WidgetDecorator::measure(self, measure_spec);
    }

    fn arrange(&mut self, position: Position) {
        WidgetDecorator::arrange(self, position);
    }

    fn set_measured_size(&mut self, size: MeasuredSize) {
        // TODO this doesn't belong in the Widget interface
        WidgetDecorator::set_measured_size(self, size)
    }

    fn update(&mut self) {
        WidgetDecorator::update(self);
    }

    fn parent_index(&self) -> usize {
        WidgetDecorator::parent_index(self)
    }

    fn set_parent(&mut self, index: usize) {
        // TODO this doesn't belong in the Widget interface
        WidgetDecorator::set_parent(self, index);
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        WidgetDecorator::test_input(self, event)
    }

    fn handle_input(&mut self, ctxt: InputContext, event: InputEvent) -> bool {
        WidgetDecorator::handle_input(self, ctxt, event)
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        WidgetDecorator::on_state_changed(self, state);
    }

    fn is_selectable(&self) -> bool {
        WidgetDecorator::is_selectable(self)
    }
}
