//! Widget and data binding utilities

use crate::prelude::WidgetData;

pub mod decorator;
pub mod wrapper;

/// Helper type to contain bound data and data changed callback.
pub struct WidgetDataHolder<W, D = ()>
where
    D: WidgetData,
{
    pub data: D,
    pub on_data_changed: fn(&mut W, &D::Data),
}

impl<W> Default for WidgetDataHolder<W, ()> {
    fn default() -> Self {
        Self {
            data: (),
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
            on_data_changed: |_, _| (),
        }
    }
}

impl<W, D> WidgetDataHolder<W, D>
where
    D: WidgetData,
{
    pub fn update(&mut self, widget: &mut W) {
        self.data
            .on_changed(|data| (self.on_data_changed)(widget, data));
    }

    pub fn reset_changed(&self) {
        self.data.reset_changed()
    }
}
