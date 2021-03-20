pub use object_chain::{Chain, ChainElement, Link};

use crate::{widgets::Widget, Canvas, WidgetRenderer};

pub mod column;

// Cell is a container struct used by the Row/Column layouts.
pub struct Cell<W>
where
    W: Widget,
{
    pub weight: Option<u32>,
    pub inner: W,
}

impl<W> Cell<W>
where
    W: Widget,
{
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            weight: None,
        }
    }

    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }
}

impl<C, W> WidgetRenderer<C> for Cell<W>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.inner.draw(canvas)
    }
}

pub trait LinearLayoutChainElement<C>
where
    C: Canvas,
{
    fn at(&self, index: usize) -> &dyn LinearLayoutCell<C>;

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell<C>;
}

pub trait LinearLayoutCell<C>: WidgetRenderer<C>
where
    C: Canvas,
{
    fn weight(&self) -> Option<u32>;

    fn widget(&self) -> &dyn Widget;

    fn widget_mut(&mut self) -> &mut dyn Widget;
}

impl<C, W> LinearLayoutCell<C> for Cell<W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    fn weight(&self) -> Option<u32> {
        self.weight
    }

    fn widget(&self) -> &dyn Widget {
        &self.inner
    }

    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.inner
    }
}

impl<C, W> LinearLayoutChainElement<C> for Chain<Cell<W>>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    fn at(&self, index: usize) -> &dyn LinearLayoutCell<C> {
        assert!(index == 0);

        &self.object
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell<C> {
        assert!(index == 0);

        &mut self.object
    }
}

impl<C, W, CE> LinearLayoutChainElement<C> for Link<Cell<W>, CE>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
    CE: LinearLayoutChainElement<C> + ChainElement,
{
    fn at(&self, index: usize) -> &dyn LinearLayoutCell<C> {
        if index == Link::len(self) - 1 {
            return &self.object;
        }

        return self.parent.at(index);
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell<C> {
        if index == Link::len(self) - 1 {
            return &mut self.object;
        }

        return self.parent.at_mut(index);
    }
}

impl<C, W> WidgetRenderer<C> for Chain<Cell<W>>
where
    W: Widget,
    Cell<W>: WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.object.draw(canvas)
    }
}

impl<C, W, CE> WidgetRenderer<C> for Link<Cell<W>, CE>
where
    W: Widget,
    CE: LinearLayoutChainElement<C> + ChainElement,
    Cell<W>: WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.object.draw(canvas)
    }
}
