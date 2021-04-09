use core::{
    cell::{RefCell, RefMut},
    ops::Deref,
};

pub trait WidgetData {
    type Data;
    type Version: PartialEq + Default;

    fn update(&self, _f: impl FnOnce(&mut Self::Data));

    fn read<W>(&self, widget: &mut W, callback: fn(&mut W, &Self::Data));

    fn version(&self) -> Self::Version;
}

impl<T> WidgetData for &T
where
    T: WidgetData,
{
    type Data = T::Data;
    type Version = T::Version;

    fn update(&self, f: impl FnOnce(&mut Self::Data)) {
        (*self).update(f)
    }

    fn read<W>(&self, widget: &mut W, callback: fn(&mut W, &Self::Data)) {
        (*self).read(widget, callback);
    }

    fn version(&self) -> Self::Version {
        (*self).version()
    }
}

pub struct NoData;

impl Default for NoData {
    fn default() -> Self {
        Self
    }
}

impl WidgetData for NoData {
    type Data = ();
    type Version = ();

    fn update(&self, _f: impl FnOnce(&mut Self::Data)) {}

    fn read<W>(&self, widget: &mut W, callback: fn(&mut W, &Self::Data)) {
        callback(widget, &());
    }

    fn version(&self) -> () {}
}

struct BoundDataInner<D, F>
where
    F: FnMut(&D),
{
    version: usize,
    data: D,
    on_changed: F,
}

/// Wraps a piece of data to be used in Widgets.
pub struct BoundData<D, F>
where
    F: FnMut(&D),
{
    inner: RefCell<BoundDataInner<D, F>>,
}

impl<D, F> BoundData<D, F>
where
    F: FnMut(&D),
{
    pub fn new(init: D, on_changed: F) -> Self {
        Self {
            inner: RefCell::new(BoundDataInner {
                version: 0,
                data: init,
                on_changed,
            }),
        }
    }
}

impl<D, F> WidgetData for BoundData<D, F>
where
    F: FnMut(&D),
{
    type Data = D;
    type Version = usize;

    fn update(&self, updater: impl FnOnce(&mut Self::Data)) {
        let mut borrow = self.inner.borrow_mut();

        borrow.version = borrow.version.wrapping_add(1);
        updater(&mut borrow.data);

        let (data, mut on_changed) =
            RefMut::map_split(borrow, |f| (&mut f.data, &mut f.on_changed));

        on_changed(data.deref());
    }

    fn read<W>(&self, widget: &mut W, callback: fn(&mut W, &Self::Data)) {
        let borrow = self.inner.borrow();
        callback(widget, &borrow.data);
    }

    fn version(&self) -> usize {
        self.inner.borrow().version
    }
}
