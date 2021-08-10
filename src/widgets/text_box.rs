use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    state::{
        selection::{Selected, Unselected},
        WidgetState,
    },
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
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub state: WidgetState,
}

impl<S, P> TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl TextBox<(), ()> {
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
