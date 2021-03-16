use crate::{
    data::{NoData, WidgetData},
    input::{InputEvent, Key},
    widgets::{DataHolder, Widget, WidgetDataHolder, WidgetProperties},
    BoundingBox, InputCtxt, MeasureSpec,
};

pub struct Button<I, D>
where
    D: WidgetData,
{
    pub widget_properties: WidgetProperties,
    pub inner: I,
    pub data_holder: WidgetDataHolder<Self, D>,
    pub on_clicked: fn(&mut D),
}

impl<I> Button<I, NoData>
where
    I: Widget,
{
    pub fn new(inner: I) -> Self {
        Self {
            widget_properties: WidgetProperties::default(),
            inner,
            data_holder: WidgetDataHolder::default(),
            on_clicked: |_| (),
        }
    }

    pub fn bind<D>(self, data: D) -> Button<I, D>
    where
        D: WidgetData,
    {
        Button {
            widget_properties: self.widget_properties,
            inner: self.inner,
            data_holder: self.data_holder.bind(data),
            on_clicked: |_| (),
        }
    }
}

impl<I, D> Button<I, D>
where
    I: Widget,
    D: WidgetData,
{
    pub fn on_clicked(self, callback: fn(&mut D)) -> Button<I, D>
    where
        Self: Sized,
    {
        Button {
            widget_properties: self.widget_properties,
            inner: self.inner,
            data_holder: self.data_holder,
            on_clicked: callback,
        }
    }

    fn fire_on_pressed(&mut self) {}
    fn fire_on_clicked(&mut self) {
        let callback = self.on_clicked;
        callback(&mut self.data_holder.data)
    }
}

impl<I, D> DataHolder for Button<I, D>
where
    I: Widget,
    D: WidgetData,
{
    type Data = D;

    fn data_holder(&mut self) -> &mut WidgetDataHolder<Self, Self::Data>
    where
        Self: Sized,
    {
        &mut self.data_holder
    }
}

impl<I, D> Widget for Button<I, D>
where
    I: Widget,
    D: WidgetData,
{
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        self.inner.bounding_box_mut()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec)
    }

    fn children(&self) -> usize {
        1 + self.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.inner
        } else {
            self.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.inner
        } else {
            self.inner.get_mut_child(idx - 1)
        }
    }

    fn handle_input(&mut self, ctxt: &mut InputCtxt, event: InputEvent) -> bool {
        if !self.inner.handle_input(ctxt, event) {
            match event {
                InputEvent::KeyUp(Key::Space, _) => self.fire_on_clicked(),
                InputEvent::KeyUp(Key::Tab, _) => ctxt.select_next_widget(),
                InputEvent::PointerHover(_) => {}
                InputEvent::PointerDown(_) => {
                    self.fire_on_pressed();
                }
                InputEvent::PointerMove(_) => {}
                InputEvent::PointerMoveDelta(_) => {}
                InputEvent::PointerUp(pos) => {
                    if self.bounding_box().hit_test(pos) {
                        self.fire_on_clicked();
                    }
                }
                _ => return false,
            }

            true
        } else {
            false
        }
    }

    fn update_impl(&mut self) {}
}
