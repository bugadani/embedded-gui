use core::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
};

struct WidgetDataInner<D, F> {
    version: usize,
    data: D,
    on_changed: F,
}

/// Wraps a piece of data to be used in Widgets.
pub struct WidgetData<D, F> {
    inner: RefCell<WidgetDataInner<D, F>>,
}

impl<D, F> WidgetData<D, F>
where
    F: FnMut(&D),
{
    pub fn new(init: D, on_changed: F) -> Self {
        Self {
            inner: RefCell::new(WidgetDataInner {
                version: 0,
                data: init,
                on_changed,
            }),
        }
    }

    pub fn update(&self, f: impl Fn(RefMut<D>) -> bool) {
        let borrow = self.inner.borrow_mut();
        let (data, mut version) = RefMut::map_split(borrow, |f| (&mut f.data, &mut f.version));
        if f(data) {
            *version = version.wrapping_add(1);
            core::mem::drop(version);

            let borrow = self.inner.borrow_mut();
            let (data, mut on_changed) =
                RefMut::map_split(borrow, |f| (&mut f.data, &mut f.on_changed));
            on_changed(data.deref());
        }
    }

    pub fn get(&self) -> Ref<D> {
        let borrow = self.inner.borrow();

        Ref::map(borrow, |f| &f.data)
    }

    pub fn version(&self) -> usize {
        self.inner.borrow().version
    }
}
