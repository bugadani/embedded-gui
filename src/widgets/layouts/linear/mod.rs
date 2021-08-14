//! Linear layouts
//!
//! Arrange widgets in a Row or a Column.
//!

pub use object_chain::{Chain, Link};

use crate::{widgets::Widget, Canvas, WidgetRenderer};

mod column;
mod layout;
mod row;

pub use column::Column;
pub use layout::LinearLayout;
pub use row::Row;

mod private;

#[derive(Copy, Clone)]
pub struct NoSpacing;
#[derive(Copy, Clone)]
pub struct WithSpacing(u32);

pub trait ElementSpacing: Copy {
    fn spacing(&self) -> u32;
}

impl ElementSpacing for NoSpacing {
    fn spacing(&self) -> u32 {
        0
    }
}

impl ElementSpacing for WithSpacing {
    fn spacing(&self) -> u32 {
        self.0
    }
}

pub trait CellWeight {
    fn weight(&self) -> u32;
}

pub struct NoWeight;

impl CellWeight for NoWeight {
    fn weight(&self) -> u32 {
        0
    }
}

pub struct Weight(u32);

impl Weight {
    pub fn new(w: u32) -> Self {
        Self(w)
    }
}

impl CellWeight for Weight {
    fn weight(&self) -> u32 {
        self.0
    }
}

// Cell is a container struct used by the Row/Column layouts.
pub struct Cell<W, CW = NoWeight>
where
    W: Widget,
{
    pub weight: CW,
    pub inner: W,
}

impl<W> Cell<W, NoWeight>
where
    W: Widget,
{
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            weight: NoWeight,
        }
    }

    pub fn weight(self, weight: u32) -> Cell<W, Weight> {
        Cell {
            inner: self.inner,
            weight: Weight::new(weight),
        }
    }
}

impl<C, W, CW> WidgetRenderer<C> for Cell<W, CW>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
    CW: CellWeight,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.inner.draw(canvas)
    }
}
