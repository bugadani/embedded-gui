pub use object_chain::{Chain, ChainElement, Link};

use crate::{
    geometry::{
        axis_order::AxisOrder,
        measurement::{MeasureConstraint, MeasureSpec},
        BoundingBox, MeasuredSize,
    },
    input::event::InputEvent,
    state::WidgetState,
    widgets::{
        layouts::linear::{Cell, CellWeight},
        Widget,
    },
    Canvas, Position, WidgetRenderer,
};

pub trait LayoutDirection: Copy {
    type AxisOrder: AxisOrder;

    fn main_axis_size(bounds: BoundingBox) -> u32 {
        <Self::AxisOrder as AxisOrder>::main_axis(bounds.size.width, bounds.size.height)
    }

    fn cross_axis_size(bounds: BoundingBox) -> u32 {
        <Self::AxisOrder as AxisOrder>::cross_axis(bounds.size.width, bounds.size.height)
    }

    fn create_measured_size(main: u32, cross: u32) -> MeasuredSize {
        let (x, y) = <Self::AxisOrder as AxisOrder>::merge(main, cross);

        MeasuredSize {
            width: x,
            height: y,
        }
    }

    fn main_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint {
        <Self::AxisOrder as AxisOrder>::main_axis(spec.width, spec.height)
    }

    fn cross_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint {
        <Self::AxisOrder as AxisOrder>::cross_axis(spec.width, spec.height)
    }

    fn create_measure_spec(main: MeasureConstraint, cross: MeasureConstraint) -> MeasureSpec {
        let (x, y) = <Self::AxisOrder as AxisOrder>::merge(main, cross);

        MeasureSpec {
            width: x,
            height: y,
        }
    }

    fn arrange(pos: Position, bb: BoundingBox, spacing: u32) -> Position {
        let increment = Self::main_axis_size(bb) + spacing;
        let (x, y) = <Self::AxisOrder as AxisOrder>::merge(increment as i32, 0);

        Position {
            x: pos.x + x,
            y: pos.y + y,
        }
    }

    fn element_spacing(&self) -> u32;
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
    fn draw(&mut self, canvas: &mut C) -> Result<(), C::Error> {
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
    fn draw(&mut self, canvas: &mut C) -> Result<(), C::Error> {
        self.parent.draw(canvas)?;
        self.object.draw(canvas)
    }
}
