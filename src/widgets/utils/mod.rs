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
    pub fn update(&mut self, widget: &mut W) {
        let current_version = self.data.version();
        if current_version != self.last_version {
            self.last_version = current_version;

            self.data.read(widget, self.on_data_changed);
        }
    }
}
