use core::{
    cell::{Cell, RefCell, RefMut},
    ops::Deref,
};

pub trait WidgetData {
    type Data;

    fn update(&self, _f: impl FnOnce(&mut Self::Data));

    fn on_changed(&self, callback: impl FnOnce(&Self::Data));

    fn with_data<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R;

    fn reset_changed(&self);
}

impl<T> WidgetData for &T
where
    T: WidgetData,
{
    type Data = T::Data;

    fn update(&self, f: impl FnOnce(&mut Self::Data)) {
        (*self).update(f)
    }

    fn on_changed(&self, callback: impl FnOnce(&Self::Data)) {
        (*self).on_changed(callback);
    }

    fn with_data<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R {
        (*self).with_data(f)
    }

    fn reset_changed(&self) {
        (*self).reset_changed();
    }
}

impl WidgetData for () {
    type Data = ();

    fn update(&self, _f: impl FnOnce(&mut Self::Data)) {}

    fn on_changed(&self, _: impl FnOnce(&Self::Data)) {}

    fn with_data<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R {
        f(&())
    }

    fn reset_changed(&self) {}
}

struct BoundDataInner<D, F>
where
    F: FnMut(&D),
{
    data: D,
    on_changed: F,
}

/// Wraps a piece of data to be used in Widgets.
pub struct BoundData<D, F>
where
    F: FnMut(&D),
{
    inner: RefCell<BoundDataInner<D, F>>,
    changed: Cell<bool>,
}

impl<D, F> BoundData<D, F>
where
    F: FnMut(&D),
{
    pub fn new(init: D, on_changed: F) -> Self {
        Self {
            inner: RefCell::new(BoundDataInner {
                data: init,
                on_changed,
            }),
            changed: Cell::new(true),
        }
    }
}

impl<D, F> WidgetData for BoundData<D, F>
where
    F: FnMut(&D),
{
    type Data = D;

    fn update(&self, updater: impl FnOnce(&mut Self::Data)) {
        let mut borrow = self.inner.borrow_mut();

        self.changed.set(true);
        updater(&mut borrow.data);

        let (data, mut on_changed) =
            RefMut::map_split(borrow, |f| (&mut f.data, &mut f.on_changed));

        on_changed(data.deref());
    }

    fn on_changed(&self, callback: impl FnOnce(&Self::Data)) {
        if self.changed.get() {
            self.with_data(callback);
        }
    }

    fn with_data<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R {
        let borrow = self.inner.borrow();
        f(&borrow.data)
    }

    fn reset_changed(&self) {
        self.changed.set(false);
    }
}
