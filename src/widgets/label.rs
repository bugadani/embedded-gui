use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    widgets::{Widget, WidgetDataHolder, WidgetStateHolder, WidgetWrapper},
    BoundingBox, Canvas, MeasureSpec, MeasuredSize, WidgetState,
};

pub trait LabelProperties<C: Canvas> {
    fn measure_text(&self, text: &str) -> MeasuredSize;
}

pub trait LabelConstructor<C, P> {
    fn new(text: &'static str) -> WidgetWrapper<Label<C, P>, NoData>
    where
        C: Canvas,
        P: LabelProperties<C>;
}

pub struct Label<C, P>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    // FIXME: use heapless::String
    pub text: &'static str,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub _marker: PhantomData<C>,
}

impl<C, P> Label<C, P>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    pub fn bind<D>(self) -> Label<C, P>
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

impl<C, P> WidgetWrapper<Label<C, P>, NoData>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    pub fn bind<D>(self, data: D) -> WidgetWrapper<Label<C, P>, D>
    where
        D: WidgetData,
    {
        WidgetWrapper {
            parent_index: self.parent_index,
            widget: self.widget,
            data_holder: WidgetDataHolder::<Label<C, P>, NoData>::default().bind(data),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

impl<C, P, D> WidgetStateHolder for WidgetWrapper<Label<C, P>, D>
where
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

impl<C, P, D> Widget for WidgetWrapper<Label<C, P>, D>
where
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
        let size = self.widget.label_properties.measure_text(self.widget.text);

        let width = measure_spec.width.apply_to_measured(size.width);
        let height = measure_spec.height.apply_to_measured(size.height);

        self.set_measured_size(MeasuredSize { width, height })
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}
