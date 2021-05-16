//! A box with a tick mark.

use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    state::WidgetState,
    widgets::{
        wrapper::{Wrapper, WrapperBindable},
        Widget,
    },
};

pub trait CheckBoxProperties {
    type Color;

    fn set_background_color(&mut self, color: Self::Color);

    fn set_border_color(&mut self, color: Self::Color);

    fn set_check_mark_color(&mut self, color: Self::Color);

    fn set_checked(&mut self, checked: bool);

    fn measure(&self, spec: MeasureSpec) -> MeasuredSize;
}

pub struct CheckBox<P>
where
    P: CheckBoxProperties,
{
    pub checkbox_properties: P,
    pub parent_index: usize,
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub bounds: BoundingBox,
}

impl<P> CheckBox<P>
where
    P: CheckBoxProperties,
{
    pub fn new() -> CheckBox<P>
    where
        P: Default,
    {
        CheckBox {
            parent_index: 0,
            checkbox_properties: P::default(),
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }

    pub fn background_color(mut self, color: P::Color) -> Self {
        self.set_background_color(color);
        self
    }

    pub fn set_background_color(&mut self, color: P::Color) {
        self.checkbox_properties.set_background_color(color);
    }

    pub fn border_color(mut self, color: P::Color) -> Self {
        self.set_border_color(color);
        self
    }

    pub fn set_border_color(&mut self, color: P::Color) {
        self.checkbox_properties.set_border_color(color);
    }

    pub fn check_mark_color(mut self, color: P::Color) -> Self {
        self.set_check_mark_color(color);
        self
    }

    pub fn set_check_mark_color(&mut self, color: P::Color) {
        self.checkbox_properties.set_check_mark_color(color);
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.set_checked(checked);
        self
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checkbox_properties.set_checked(checked);
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<P> WrapperBindable for CheckBox<P> where P: CheckBoxProperties {}

impl<P, D> Wrapper<CheckBox<P>, D>
where
    P: CheckBoxProperties,
    D: WidgetData,
{
    pub fn on_state_changed(mut self, callback: fn(&mut CheckBox<P>, WidgetState)) -> Self {
        self.widget.on_state_changed = callback;
        self
    }
}

impl<P> Widget for CheckBox<P>
where
    P: CheckBoxProperties,
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.bounds.size = self.checkbox_properties.measure(measure_spec);
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
    }

    fn is_selectable(&self) -> bool {
        true
    }
}
