use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, Position},
    input::{controller::InputContext, event::InputEvent},
    state::WidgetState,
};

pub mod button;
pub mod graphical;
pub mod label;
pub mod layouts;
pub mod primitives;
pub mod scroll;
pub mod slider;
pub mod text_block;
pub mod text_box;
pub mod toggle;
pub mod utils;
pub mod wrapper;

pub trait Widget {
    fn attach(&mut self, parent: usize, index: usize) {
        debug_assert!(index == 0 || parent != index);
        debug_assert!(
            self.children() == 0,
            "Attach must be implemented by non-leaf widgets"
        );
        self.set_parent(parent);
    }

    fn bounding_box(&self) -> BoundingBox;

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

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

    fn update(&mut self) {}

    fn parent_index(&self) -> usize;

    fn set_parent(&mut self, _index: usize) {}

    fn test_input(&mut self, _event: InputEvent) -> Option<usize> {
        None
    }

    fn handle_input(&mut self, _ctxt: InputContext, _event: InputEvent) -> bool {
        false
    }

    fn on_state_changed(&mut self, state: WidgetState);

    fn is_selectable(&self) -> bool {
        false
    }
}

pub struct WidgetDataHolder<W, D = ()>
where
    D: WidgetData,
{
    pub data: D,
    pub last_version: D::Version,
    pub on_data_changed: fn(&mut W, &D::Data),
}

impl<W> Default for WidgetDataHolder<W, ()> {
    fn default() -> Self {
        Self {
            data: (),
            last_version: (),
            on_data_changed: |_, _| (),
        }
    }
}

impl<W> WidgetDataHolder<W, ()> {
    pub fn new<D>(data: D) -> WidgetDataHolder<W, D>
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
