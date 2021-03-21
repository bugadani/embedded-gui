use core::marker::PhantomData;

use crate::{
    data::{NoData, WidgetData},
    input::{controller::InputContext, event::InputEvent},
    BoundingBox, MeasureSpec, MeasuredSize, Position, WidgetState,
};

pub mod button;
pub mod label;
pub mod layouts;
pub mod primitives;

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

pub struct NoDataHolder<W> {
    _marker: PhantomData<W>,
    _no_data: NoData,
}

impl<W> Default for NoDataHolder<W> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            _no_data: NoData::default(),
        }
    }
}

impl<W> NoDataHolder<W> {
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

impl<W> WidgetDataHolderTrait for NoDataHolder<W> {
    type Owner = W;
    type Data = NoData;

    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self._no_data
    }
    fn update(&mut self, _widget: &mut Self::Owner) {}
    fn on_data_changed(
        &mut self,
        _callback: fn(&mut Self::Owner, &<Self::Data as WidgetData>::Data),
    ) {
    }
}

pub trait WidgetDataHolderTrait {
    type Owner;
    type Data: WidgetData;

    fn data_mut(&mut self) -> &mut Self::Data;

    fn update(&mut self, widget: &mut Self::Owner);

    fn on_data_changed(
        &mut self,
        _callback: fn(&mut Self::Owner, &<Self::Data as WidgetData>::Data),
    );
}

pub struct WidgetDataHolder<W, D>
where
    D: WidgetData,
{
    pub data: D,
    pub last_version: usize,
    pub on_data_changed: fn(&mut W, &D::Data),
}

impl<W, D> WidgetDataHolderTrait for WidgetDataHolder<W, D>
where
    D: WidgetData,
{
    type Owner = W;
    type Data = D;

    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }

    fn update(&mut self, widget: &mut Self::Owner) {
        let current_version = self.data.version();
        if current_version != self.last_version {
            self.last_version = current_version;

            self.data.read(widget, self.on_data_changed);
        }
    }

    fn on_data_changed(
        &mut self,
        callback: fn(&mut Self::Owner, &<Self::Data as WidgetData>::Data),
    ) {
        self.on_data_changed = callback;
    }
}

pub trait DataHolder {
    type Data: WidgetData;
    type Widget;

    fn on_data_changed(
        self,
        callback: fn(&mut Self::Widget, &<Self::Data as WidgetData>::Data),
    ) -> Self
    where
        Self: Sized;
}

pub trait WidgetStateHolder {
    fn change_state(&mut self, state: u32);
    fn change_selection(&mut self, state: bool);
    fn is_selectable(&self) -> bool {
        true
    }
}

pub struct WidgetWrapper<W, D = NoDataHolder<W>>
where
    D: WidgetDataHolderTrait<Owner = W>,
{
    pub parent_index: usize,
    pub widget: W,
    pub data_holder: D,
    pub state: WidgetState,
    pub on_state_changed: fn(&mut W, WidgetState),
}

impl<W> WidgetWrapper<W, NoDataHolder<W>> {
    pub fn new(widget: W) -> Self {
        WidgetWrapper {
            parent_index: 0,
            widget,
            data_holder: NoDataHolder::<W>::default(),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

pub trait UpdateHandler {
    fn update(&mut self) {}
}

impl<W, D> WidgetWrapper<W, D>
where
    D: WidgetDataHolderTrait<Owner = W>,
{
    pub fn on_state_changed(mut self, callback: fn(&mut W, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }

    pub fn apply(&mut self, func: impl FnOnce(&mut W)) {
        func(&mut self.widget);
    }
}

impl<W, DH> DataHolder for WidgetWrapper<W, DH>
where
    DH: WidgetDataHolderTrait<Owner = W>,
{
    type Data = DH::Data;
    type Widget = W;

    fn on_data_changed(
        mut self,
        callback: fn(&mut Self::Widget, &<Self::Data as WidgetData>::Data),
    ) -> Self
    where
        Self: Sized,
    {
        self.data_holder.on_data_changed(callback);
        self
    }
}

impl<W, DH> UpdateHandler for WidgetWrapper<W, DH>
where
    DH: WidgetDataHolderTrait<Owner = W>,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}

pub trait ParentHolder {
    fn parent_index(&self) -> usize;

    fn set_parent(&mut self, index: usize);
}

impl<W, D> ParentHolder for WidgetWrapper<W, D>
where
    D: WidgetDataHolderTrait<Owner = W>,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }
    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}
