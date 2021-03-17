use crate::{
    data::{NoData, WidgetData},
    input::{InputEvent, Key},
    widgets::{DataHolder, Widget, WidgetDataHolder, WidgetProperties},
    BoundingBox, InputCtxt, MeasureSpec,
};

pub struct ButtonWidget<I, D>
where
    D: WidgetData,
{
    pub widget_properties: WidgetProperties,
    pub inner: I,
    pub on_clicked: fn(&mut D),
}

impl<I> ButtonWidget<I, NoData>
where
    I: Widget,
{
    pub fn new(inner: I) -> Self {
        Self {
            widget_properties: WidgetProperties::default(),
            inner,
            on_clicked: |_| (),
        }
    }

    pub fn bind<D>(self) -> ButtonWidget<I, D>
    where
        D: WidgetData,
    {
        ButtonWidget {
            widget_properties: self.widget_properties,
            inner: self.inner,
            on_clicked: |_| (),
        }
    }
}

impl<I, D> ButtonWidget<I, D>
where
    I: Widget,
    D: WidgetData,
{
    pub fn on_clicked(mut self, callback: fn(&mut D)) -> Self
    where
        D: WidgetData,
    {
        self.on_clicked = callback;
        self
    }
}

pub struct Button<I, D>
where
    D: WidgetData,
{
    pub widget: ButtonWidget<I, D>,
    pub data_holder: WidgetDataHolder<ButtonWidget<I, D>, D>,
}

impl<I> Button<I, NoData>
where
    I: Widget,
{
    pub fn new(inner: I) -> Self {
        Self {
            widget: ButtonWidget::new(inner),
            data_holder: WidgetDataHolder::default(),
        }
    }

    pub fn bind<D>(self, data: D) -> Button<I, D>
    where
        D: WidgetData,
    {
        Button {
            widget: self.widget.bind::<D>(),
            data_holder: self.data_holder.bind(data),
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
            widget: self.widget.on_clicked(callback),
            data_holder: self.data_holder,
        }
    }

    fn fire_on_pressed(&mut self) {}
    fn fire_on_clicked(&mut self) {
        let callback = self.widget.on_clicked;
        callback(&mut self.data_holder.data)
    }
}

impl<I, D> DataHolder for Button<I, D>
where
    I: Widget,
    D: WidgetData,
{
    type Data = D;
    type Widget = ButtonWidget<I, D>;

    fn data_holder(&mut self) -> &mut WidgetDataHolder<Self::Widget, Self::Data>
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
        &mut self.widget.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.widget.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        self.widget.inner.bounding_box_mut()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.widget.inner.measure(measure_spec)
    }

    fn children(&self) -> usize {
        1 + self.widget.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.widget.inner
        } else {
            self.widget.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.widget.inner
        } else {
            self.widget.inner.get_mut_child(idx - 1)
        }
    }

    fn handle_input(&mut self, ctxt: &mut InputCtxt, event: InputEvent) -> bool {
        if !self.widget.inner.handle_input(ctxt, event) {
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

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}
