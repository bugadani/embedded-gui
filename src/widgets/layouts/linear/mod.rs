pub use object_chain::{Chain, ChainElement, Link};

use crate::{
    input::event::InputEvent,
    state::WidgetState,
    widgets::{layouts::linear::layout::LayoutDirection, Widget},
    Canvas, Position, WidgetRenderer,
};

pub mod column;
pub mod layout;
pub mod row;

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

pub trait LinearLayoutChainElement {
    fn at(&self, index: usize) -> &dyn LinearLayoutCell;

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell;

    fn test_input(&mut self, event: InputEvent) -> Option<usize>;

    fn count_widgets(&self) -> usize;

    fn arrange<L>(&mut self, position: Position, direction: L, spacing: u32) -> Position
    where
        L: LayoutDirection;

    fn on_state_changed(&mut self, state: WidgetState);

    fn update(&mut self);
}

impl<W, CW> LinearLayoutChainElement for Chain<Cell<W, CW>>
where
    W: Widget,
    CW: CellWeight,
{
    fn at(&self, index: usize) -> &dyn LinearLayoutCell {
        debug_assert!(index == 0);

        &self.object
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell {
        debug_assert!(index == 0);

        &mut self.object
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.object.inner.test_input(event)
    }

    fn count_widgets(&self) -> usize {
        self.object.inner.children() + 1
    }

    fn arrange<L>(&mut self, position: Position, _direction: L, spacing: u32) -> Position
    where
        L: LayoutDirection,
    {
        self.object.inner.arrange(position);

        L::arrange(position, self.object.inner.bounding_box(), spacing)
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.object.inner.on_state_changed(state);
    }

    fn update(&mut self) {
        self.object.inner.update();
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

        self.parent.at(index)
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn LinearLayoutCell {
        if index == Link::len(self) - 1 {
            return &mut self.object;
        }

        self.parent.at_mut(index)
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.parent.test_input(event).or_else(|| {
            self.object
                .inner
                .test_input(event)
                .map(|idx| idx + self.parent.count_widgets())
        })
    }

    fn count_widgets(&self) -> usize {
        self.object.inner.children() + 1 + self.parent.count_widgets()
    }

    fn arrange<L>(&mut self, position: Position, direction: L, spacing: u32) -> Position
    where
        L: LayoutDirection,
    {
        let next_pos = self.parent.arrange(position, direction, spacing);

        self.object.inner.arrange(next_pos);

        L::arrange(next_pos, self.object.inner.bounding_box(), spacing)
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.object.inner.on_state_changed(state);
        self.parent.on_state_changed(state);
    }

    fn update(&mut self) {
        self.object.inner.update();
        self.parent.update();
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
