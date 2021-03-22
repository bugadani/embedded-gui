use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    widgets::{container::Container, Widget, WidgetStateHolder},
    BoundingBox, Canvas, MeasureSpec, MeasuredSize, WidgetState,
};

pub trait LabelProperties<C: Canvas> {
    fn measure_text(&self, text: &str) -> MeasuredSize;
}

pub struct Label<S, C, P>
where
    S: AsRef<str>,
    C: Canvas,
    P: LabelProperties<C>,
{
    pub text: S,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub _marker: PhantomData<C>,
}

impl<S, C, P> Label<S, C, P>
where
    S: AsRef<str>,
    C: Canvas,
    P: LabelProperties<C>,
{
    pub fn bind<D>(self) -> Label<S, C, P>
    where
        D: WidgetData,
    {
        Label {
            label_properties: self.label_properties,
            bounds: self.bounds,
            text: self.text,
            _marker: PhantomData,
        }
    }
}

impl<S, C, P> Container<Label<S, C, P>, NoData>
where
    S: AsRef<str>,
    C: Canvas,
    P: LabelProperties<C>,
{
    pub fn bind<D>(self, data: D) -> Container<Label<S, C, P>, D>
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

impl<S, C, P, D> WidgetStateHolder for Container<Label<S, C, P>, D>
where
    S: AsRef<str>,
    C: Canvas,
    P: LabelProperties<C>,
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

impl<S, C, P, D> Widget for Container<Label<S, C, P>, D>
where
    S: AsRef<str>,
    C: Canvas,
    P: LabelProperties<C>,
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
