pub use object_chain::{Chain, ChainElement, Link};

use crate::{widgets::Widget, Canvas, WidgetRenderer};

pub mod column;
pub mod row;

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

pub trait LinearLayoutChainElement {
    fn at(&self, index: usize) -> &dyn LinearLayoutCell;

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell;
}

pub trait LinearLayoutCell {
    fn weight(&self) -> u32;

    fn widget(&self) -> &dyn Widget;

    fn widget_mut(&mut self) -> &mut dyn Widget;
}

impl<W, CW> LinearLayoutCell for Cell<W, CW>
where
    W: Widget,
    CW: CellWeight,
{
    fn weight(&self) -> u32 {
        self.weight.weight()
    }

    fn widget(&self) -> &dyn Widget {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.inner
    }
}

impl<W, CW> LinearLayoutChainElement for Chain<Cell<W, CW>>
where
    W: Widget,
    CW: CellWeight,
{
    fn at(&self, index: usize) -> &dyn LinearLayoutCell {
        assert!(index == 0);

        &self.object
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell {
        assert!(index == 0);

        &mut self.object
    }
}

impl<W, CE, CW> LinearLayoutChainElement for Link<Cell<W, CW>, CE>
where
    W: Widget,
    CE: LinearLayoutChainElement + ChainElement,
    CW: CellWeight,
{
    fn at(&self, index: usize) -> &dyn LinearLayoutCell {
        if index == Link::len(self) - 1 {
            return &self.object;
        }

        return self.parent.at(index);
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell {
        if index == Link::len(self) - 1 {
            return &mut self.object;
        }

        return self.parent.at_mut(index);
    }
}

impl<C, W, CW> WidgetRenderer<C> for Chain<Cell<W, CW>>
where
    W: Widget,
    Cell<W, CW>: WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.object.draw(canvas)
    }
}

impl<C, W, CE, CW> WidgetRenderer<C> for Link<Cell<W, CW>, CE>
where
    W: Widget,
    CE: LinearLayoutChainElement + ChainElement + WidgetRenderer<C>,
    Cell<W, CW>: WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.parent.draw(canvas)?;
        self.object.draw(canvas)
    }
}
