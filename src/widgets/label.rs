use crate::{
    data::{NoData, WidgetData},
    widgets::{container::Container, Widget, WidgetStateHolder},
    BoundingBox, Canvas, MeasureSpec, MeasuredSize, WidgetState,
};

pub trait LabelProperties {
    type Canvas: Canvas;

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
}

impl<S, P> Label<S, P>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    pub fn bind<D>(self) -> Label<S, P>
    where
        D: WidgetData,
    {
        Label {
            label_properties: self.label_properties,
            bounds: self.bounds,
            text: self.text,
        }
    }
}

impl<S, P> Container<Label<S, P>, NoData>
where
    S: AsRef<str>,
    P: LabelProperties,
{
    pub fn bind<D>(self, data: D) -> Container<Label<S, P>, D>
    where
        D: WidgetData,
    {
        Container {
            parent_index: self.parent_index,
            widget: self.widget,
            data_holder: self.data_holder.bind(data),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

impl<S, P, D> WidgetStateHolder for Container<Label<S, P>, D>
where
    S: AsRef<str>,
    P: LabelProperties,
    D: WidgetData,
{
    fn change_state(&mut self, state: u32) {
        // apply state
        if self.state.change_state(state) {
            (self.on_state_changed)(&mut self.widget, self.state);
        }
    }

    fn change_selection(&mut self, state: bool) {
        // apply state
        if self.state.change_selection(state) {
            (self.on_state_changed)(&mut self.widget, self.state);
        }
    }

    fn is_selectable(&self) -> bool {
        true
    }
}

impl<S, P, D> Widget for Container<Label<S, P>, D>
where
    S: AsRef<str>,
    P: LabelProperties,
    D: WidgetData,
{
    fn bounding_box(&self) -> BoundingBox {
        self.widget.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.widget.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self
            .widget
            .label_properties
            .measure_text(self.widget.text.as_ref());

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.set_measured_size(MeasuredSize { width, height })
    }
}
