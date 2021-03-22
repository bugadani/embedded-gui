use crate::{
    data::{NoData, WidgetData},
    widgets::{ParentHolder, UpdateHandler, WidgetDataHolder},
    WidgetState,
};

pub struct Container<W, D = NoData>
where
    D: WidgetData,
{
    pub parent_index: usize,
    pub widget: W,
    pub data_holder: WidgetDataHolder<W, D>,
    pub state: WidgetState,
    pub on_state_changed: fn(&mut W, WidgetState),
}

impl<W> Container<W, NoData> {
    pub fn new(widget: W) -> Self {
        Container {
            parent_index: 0,
            widget,
            data_holder: WidgetDataHolder::default(),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

impl<W, D> Container<W, D>
where
    D: WidgetData,
{
    pub fn on_state_changed(mut self, callback: fn(&mut W, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }

    pub fn on_data_changed(mut self, callback: fn(&mut W, &D::Data)) -> Self {
        self.data_holder.on_data_changed = callback;
        self
    }
}

impl<W, D> UpdateHandler for Container<W, D>
where
    D: WidgetData,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }
}

impl<W, D> ParentHolder for Container<W, D>
where
    D: WidgetData,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }
    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}
