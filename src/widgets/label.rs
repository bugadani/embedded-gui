use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    state::WidgetState,
    widgets::{wrapper::Wrapper, ParentHolder, UpdateHandler, Widget, WidgetStateHolder},
};

pub trait LabelProperties {
    fn measure_text(&self, text: &str) -> MeasuredSize;
}

pub struct Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    pub text: S,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub state: WidgetState,
}

impl<S, P> Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
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

impl<S, P> WidgetStateHolder for Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    fn change_state(&mut self, state: u32) {
        // apply state
        if self.state.change_state(state) {
            (self.on_state_changed)(self, self.state);
        }
    }

    fn change_selection(&mut self, state: bool) {
        // apply state
        if self.state.change_selection(state) {
            (self.on_state_changed)(self, self.state);
        }
    }

    fn is_selectable(&self) -> bool {
        true
    }
}

impl<S, P> Widget for Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self.label_properties.measure_text(self.text.as_ref());

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.set_measured_size(MeasuredSize { width, height })
    }
}

impl<S, P> UpdateHandler for Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    fn update(&mut self) {}
}

impl<S, P> ParentHolder for Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}
