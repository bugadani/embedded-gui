use crate::{
    data::{NoData, WidgetData},
    input::{InputEvent, Key},
    BoundingBox, InputCtxt, MeasureSpec, MeasuredSize, Position, Size,
};

pub mod button;
pub mod label;

pub trait Widget {
    //type InputController: InputController;

    //fn input_event(&mut self, event: <Self::InputController as InputController>::Event) -> bool;
    fn widget_properties(&mut self) -> &mut WidgetProperties;

    fn bounding_box(&self) -> BoundingBox;

    fn bounding_box_mut(&mut self) -> &mut BoundingBox;

    fn children(&self) -> usize {
        0
    }

    fn get_child(&self, _idx: usize) -> &dyn Widget {
        unimplemented!()
    }

    fn get_mut_child(&mut self, _idx: usize) -> &mut dyn Widget {
        unimplemented!()
    }

    fn width(mut self, width: Size) -> Self
    where
        Self: Sized,
    {
        self.widget_properties().width = width;
        self
    }

    fn height(mut self, height: Size) -> Self
    where
        Self: Sized,
    {
        self.widget_properties().height = height;
        self
    }

    fn measure(&mut self, measure_spec: MeasureSpec);

    fn arrange(&mut self, position: Position) {
        self.bounding_box_mut().position = position;
    }

    fn set_measured_size(&mut self, size: MeasuredSize) {
        self.bounding_box_mut().size = size;
    }

    fn handle_input(&mut self, ctxt: &mut InputCtxt, event: InputEvent) -> bool {
        if matches!(event, InputEvent::KeyDown(Key::Tab, _, _)) {
            ctxt.select_next_widget();
            true
        } else {
            false
        }
    }

    fn hit_test(&self, position: Position) -> Option<usize> {
        if self.bounding_box().hit_test(position) {
            if self.children() > 0 {
                let mut index = 0;
                for i in 0..self.children() {
                    let child = self.get_child(i);
                    if let Some(idx) = child.hit_test(position) {
                        return Some(index + idx);
                    }
                    index += child.children();
                }
            }

            Some(0)
        } else {
            None
        }
    }

    fn update(&mut self) {}
}

pub struct WidgetProperties {
    pub width: Size,
    pub height: Size,
}

impl Default for WidgetProperties {
    fn default() -> Self {
        Self {
            width: Size::WrapContent,
            height: Size::WrapContent,
        }
    }
}

pub struct WidgetDataHolder<W, D>
where
    D: WidgetData,
{
    pub data: D,
    pub last_version: usize,
    pub on_data_changed: fn(&mut W, &D),
}

impl<W> Default for WidgetDataHolder<W, NoData> {
    fn default() -> Self {
        Self {
            data: NoData::default(),
            last_version: 0,
            on_data_changed: |_, _| (),
        }
    }
}

impl<W> WidgetDataHolder<W, NoData> {
    pub fn bind<W2, D>(self, data: D) -> WidgetDataHolder<W2, D>
    where
        D: WidgetData,
    {
        WidgetDataHolder {
            data,
            last_version: 0,
            on_data_changed: |_, _| (),
        }
    }
}

impl<W, D> WidgetDataHolder<W, D>
where
    D: WidgetData,
{
    fn update(&mut self, widget: &mut W) {
        let current_version = self.data.version();
        if current_version != self.last_version {
            self.last_version = current_version;

            let callback = self.on_data_changed;
            callback(widget, &self.data);
        }
    }
}

pub trait DataHolder: Widget {
    type Data: WidgetData;
    type Widget;

    fn data_holder(&mut self) -> &mut WidgetDataHolder<Self::Widget, Self::Data>;

    fn on_data_changed(mut self, callback: fn(&mut Self::Widget, &Self::Data)) -> Self
    where
        Self: Sized,
    {
        self.data_holder().on_data_changed = callback;
        self
    }
}
