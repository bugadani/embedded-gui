//! Text block widget to display text

use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    state::WidgetState,
    widgets::{utils::wrapper::WrapperBindable, Widget},
};

pub trait TextBlockProperties {
    fn measure_text(&self, text: &str, spec: MeasureSpec) -> MeasuredSize;
}

pub struct TextBlock<S, P> {
    pub text: S,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub on_state_changed: fn(&mut Self, WidgetState),
}

impl<S, P> TextBlock<S, P>
where
    S: AsRef<str>,
    P: TextBlockProperties,
{
    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<S, P> Widget for TextBlock<S, P>
where
    S: AsRef<str>,
    P: TextBlockProperties,
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

    fn on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
    }

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<S, P> WrapperBindable for TextBlock<S, P>
where
    S: AsRef<str>,
    P: TextBlockProperties,
{
}
