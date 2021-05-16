use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    state::WidgetState,
    widgets::{
        wrapper::{Wrapper, WrapperBindable},
        ParentHolder, UpdateHandler, Widget, WidgetStateHolder,
    },
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

    pub fn bind<D>(self, data: D) -> Wrapper<Self, D>
    where
        D: WidgetData,
    {
        Wrapper::wrap(self, data)
    }
}

impl<S, P> WidgetStateHolder for TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
    fn on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
    }

    fn is_selectable(&self) -> bool {
        true
    }
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

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self
            .label_properties
            .measure_text(self.text.as_ref(), measure_spec);

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.set_measured_size(MeasuredSize { width, height })
    }
}

impl<S, P> UpdateHandler for TextBox<S, P> {}

impl<S, P> WrapperBindable for TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
}

impl<S, P> ParentHolder for TextBox<S, P>
where
    S: AsRef<str>,
    P: TextBoxProperties,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}
