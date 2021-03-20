use core::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
};

pub trait WidgetData {
    type Data;

    fn update(&self, _f: impl Fn(&mut Self::Data));

    fn read(&self) -> Ref<Self::Data>;

    fn version(&self) -> usize;
}

impl<T> WidgetData for &T
where
    T: WidgetData,
{
    type Data = T::Data;

    fn update(&self, f: impl Fn(&mut Self::Data)) {
        (*self).update(f)
    }

    fn read(&self) -> Ref<Self::Data> {
        (*self).read()
    }

    fn version(&self) -> usize {
        (*self).version()
    }
}

pub struct NoData {
    cell: RefCell<()>,
}

impl Default for NoData {
    fn default() -> Self {
        Self {
            cell: RefCell::new(()),
        }
    }
}

impl WidgetData for NoData {
    type Data = ();

    fn update(&self, _f: impl Fn(&mut Self::Data)) {}

    fn read(&self) -> Ref<Self::Data> {
        self.cell.borrow()
    }

    fn version(&self) -> usize {
        0
    }
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

    fn update(&self, updater: impl Fn(&mut Self::Data)) {
        let mut borrow = self.inner.borrow_mut();

        borrow.version = borrow.version.wrapping_add(1);
        updater(&mut borrow.data);

        let (data, mut on_changed) =
            RefMut::map_split(borrow, |f| (&mut f.data, &mut f.on_changed));

        on_changed(data.deref());
    }

    fn read(&self) -> Ref<Self::Data> {
        let borrow = self.inner.borrow();

        Ref::map(borrow, |f| &f.data)
    }

    fn version(&self) -> usize {
        self.inner.borrow().version
    }
}
