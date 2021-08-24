//! A radio button.

use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    state::WidgetState,
    widgets::{
        utils::wrapper::{Wrapper, WrapperBindable},
        Widget,
    },
};

pub trait RadioButtonProperties {
    type Color;

    fn set_background_color(&mut self, color: Self::Color);

    fn set_border_color(&mut self, color: Self::Color);

    fn set_check_mark_color(&mut self, color: Self::Color);

    fn set_selected(&mut self, selected: bool);

    fn measure(&self, spec: MeasureSpec) -> MeasuredSize;
}

pub struct RadioButton<P>
where
    P: RadioButtonProperties,
{
    pub radio_properties: P,
    pub parent_index: usize,
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub bounds: BoundingBox,
}

impl<P> RadioButton<P>
where
    P: RadioButtonProperties,
{
    pub fn with_style(style: P) -> Self {
        Self {
            parent_index: 0,
            radio_properties: style,
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }

    pub fn new() -> RadioButton<P>
    where
        P: Default,
    {
        Self::with_style(P::default())
    }

    pub fn background_color(mut self, color: P::Color) -> Self {
        self.set_background_color(color);
        self
    }

    pub fn set_background_color(&mut self, color: P::Color) {
        self.radio_properties.set_background_color(color);
    }

    pub fn border_color(mut self, color: P::Color) -> Self {
        self.set_border_color(color);
        self
    }

    pub fn set_border_color(&mut self, color: P::Color) {
        self.radio_properties.set_border_color(color);
    }

    pub fn check_mark_color(mut self, color: P::Color) -> Self {
        self.set_check_mark_color(color);
        self
    }

    pub fn set_check_mark_color(&mut self, color: P::Color) {
        self.radio_properties.set_check_mark_color(color);
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.set_selected(selected);
        self
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.radio_properties.set_selected(selected);
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<P> WrapperBindable for RadioButton<P> where P: RadioButtonProperties {}

impl<P, D> Wrapper<RadioButton<P>, D>
where
    P: RadioButtonProperties,
    D: WidgetData,
{
    pub fn on_state_changed(mut self, callback: fn(&mut RadioButton<P>, WidgetState)) -> Self {
        self.widget.on_state_changed = callback;
        self
    }
}

impl<P> Widget for RadioButton<P>
where
    P: RadioButtonProperties,
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.bounds.size = self.radio_properties.measure(measure_spec);
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
