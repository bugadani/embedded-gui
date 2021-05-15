use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, Position},
    input::event::InputEvent,
    state::WidgetState,
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
    Canvas, WidgetRenderer,
};

pub struct Wrapper<W, D>
where
    D: WidgetData,
{
    pub widget: W,
    pub data_holder: WidgetDataHolder<W, D>,
}

impl Wrapper<(), ()> {
    pub fn wrap<W: Widget, D: WidgetData>(widget: W, data: D) -> Wrapper<W, D> {
        Wrapper {
            widget,
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<W, D> Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn on_data_changed(mut self, callback: fn(&mut W, &D::Data)) -> Self {
        self.data_holder.on_data_changed = callback;
        self
    }
}

impl<W, D> UpdateHandler for Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
        self.widget.update();
    }
}

impl<W, D> ParentHolder for Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn parent_index(&self) -> usize {
        self.widget.parent_index()
    }

    fn set_parent(&mut self, _index: usize) {}
}

impl<W, D> Widget for Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn attach(&mut self, parent: usize, index: usize) {
        self.widget.attach(parent, index);
    }

    fn arrange(&mut self, position: Position) {
        self.widget.arrange(position);
    }

    fn bounding_box(&self) -> BoundingBox {
        self.widget.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.widget.measure(measure_spec);
    }

    fn children(&self) -> usize {
        self.widget.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        self.widget.get_child(idx)
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        self.widget.get_mut_child(idx)
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.widget.test_input(event)
    }
}

impl<W, D> WidgetStateHolder for Wrapper<W, D>
where
    W: Widget,
    D: WidgetData,
{
    fn on_state_changed(&mut self, state: WidgetState) {
        self.widget.on_state_changed(state);
    }
}

impl<W, D, C> WidgetRenderer<C> for Wrapper<W, D>
where
    W: Widget + WidgetRenderer<C>,
    D: WidgetData,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.widget.draw(canvas)
    }
}
