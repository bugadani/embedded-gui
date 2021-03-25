use crate::{
    data::{NoData, WidgetData},
    input::{controller::InputContext, event::InputEvent},
    BoundingBox, MeasureSpec, MeasuredSize, Position,
};

pub mod button;
pub mod container;
pub mod label;
pub mod layouts;
pub mod primitives;
pub mod wrapper;

pub trait Widget: WidgetStateHolder + ParentHolder + UpdateHandler {
    fn attach(&mut self, parent: usize, _index: usize) {
        self.set_parent(parent);
    }

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

    fn measure(&mut self, measure_spec: MeasureSpec);

    fn arrange(&mut self, position: Position) {
        debug_assert!(
            self.children() == 0,
            "Arrange must be implemented by non-leaf widgets"
        );
        self.bounding_box_mut().position = position;
    }

    fn set_measured_size(&mut self, size: MeasuredSize) {
        self.bounding_box_mut().size = size;
    }

    fn test_input(&mut self, _event: InputEvent) -> Option<usize> {
        None
    }

    fn handle_input(&mut self, _ctxt: InputContext, _event: InputEvent) -> bool {
        false
    }
}

pub struct WidgetDataHolder<W, D = NoData>
where
    D: WidgetData,
{
    pub data: D,
    pub last_version: D::Version,
    pub on_data_changed: fn(&mut W, &D::Data),
}

impl<W> Default for WidgetDataHolder<W, NoData> {
    fn default() -> Self {
        Self {
            data: NoData::default(),
            last_version: (),
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
            last_version: D::Version::default(),
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

            self.data.read(widget, self.on_data_changed);
        }
    }
}

pub trait WidgetStateHolder {
    fn change_state(&mut self, state: u32);
    fn change_selection(&mut self, state: bool);
    fn is_selectable(&self) -> bool {
        true
    }
}

pub trait UpdateHandler {
    fn update(&mut self) {}
}

pub trait ParentHolder {
    fn parent_index(&self) -> usize;

    fn set_parent(&mut self, index: usize);
}
