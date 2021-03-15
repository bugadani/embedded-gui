#![no_std]

pub mod widgets;

use crate::widgets::Widget;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct MeasuredSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub position: Position,
    pub size: MeasuredSize,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            size: MeasuredSize {
                width: 0,
                height: 0,
            },
        }
    }
}

pub trait WidgetRenderer<C: Canvas>: Widget {
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error>;
}

pub struct NoRenderer;

pub enum NoCanvas {}
impl Canvas for NoCanvas {
    type Error = ();
}

pub trait Canvas {
    type Error;
}

pub enum MeasureConstraint {
    AtMost(u32),
    Exactly(u32),
    Unspecified,
}

pub struct MeasureSpec {
    width: MeasureConstraint,
    height: MeasureConstraint,
}

pub enum Size {
    WrapContent,
    FillParent,
    Exact(u32),
}

pub struct Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub canvas: C,
    pub root: W,
}

impl<C, W> Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub fn new(canvas: C, root: W) -> Self {
        Self { canvas, root }
    }

    pub fn measure(&mut self) {
        self.root.measure(MeasureSpec {
            width: MeasureConstraint::AtMost(0),
            height: MeasureConstraint::AtMost(0),
        });
    }

    pub fn arrange(&mut self) {
        self.root.arrange(Position { x: 0, y: 0 });
    }

    pub fn draw(&mut self) -> Result<(), C::Error> {
        self.root.draw(&mut self.canvas)
    }
}
