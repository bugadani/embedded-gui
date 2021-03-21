#![no_std]

pub mod data;
pub mod input;
pub mod widgets;

use crate::{
    data::WidgetData,
    input::{
        controller::{DefaultInputController, InputController},
        event::InputEvent,
    },
    widgets::{Widget, WidgetWrapper},
};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct PositionDelta {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct MeasuredSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub position: Position,
    pub size: MeasuredSize,
}

impl BoundingBox {
    pub fn contains(&self, position: Position) -> bool {
        position.x >= self.position.x
            && position.y >= self.position.y
            && position.x <= self.position.x + self.size.width as i32
            && position.y <= self.position.y + self.size.height as i32
    }
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

pub trait WidgetRenderer<C: Canvas> {
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error>;
}

impl<C, W, D> WidgetRenderer<C> for WidgetWrapper<W, D>
where
    C: Canvas,
    D: WidgetData,
    W: WidgetRenderer<C>,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.widget.draw(canvas)
    }
}

pub trait Canvas {
    type Error;

    fn size(&self) -> MeasuredSize;
}

#[derive(Copy, Clone, Debug)]
pub enum MeasureConstraint {
    AtMost(u32),
    Exactly(u32),
    Unspecified,
}

impl MeasureConstraint {
    pub fn shrink(self, by: u32) -> MeasureConstraint {
        match self {
            MeasureConstraint::AtMost(size) => MeasureConstraint::AtMost(size.saturating_sub(by)),
            MeasureConstraint::Exactly(size) => MeasureConstraint::Exactly(size.saturating_sub(by)),
            MeasureConstraint::Unspecified => MeasureConstraint::Unspecified,
        }
    }

    pub fn apply_to_measured(self, measured: u32) -> u32 {
        match self {
            MeasureConstraint::AtMost(constraint) => constraint.min(measured),
            MeasureConstraint::Exactly(constraint) => constraint,
            MeasureConstraint::Unspecified => measured,
        }
    }

    pub fn to_at_most(self) -> MeasureConstraint {
        match self {
            MeasureConstraint::AtMost(size) => MeasureConstraint::AtMost(size),
            MeasureConstraint::Exactly(size) => MeasureConstraint::AtMost(size),
            MeasureConstraint::Unspecified => MeasureConstraint::AtMost(u32::MAX),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MeasureSpec {
    width: MeasureConstraint,
    height: MeasureConstraint,
}

#[derive(Copy, Clone, Default)]
pub struct WidgetState(u32);

impl WidgetState {
    const SELECTION_STATE_BITS: u32 = 0x8000_0000;
    pub fn selected(self) -> bool {
        (self.0 & Self::SELECTION_STATE_BITS) != 0
    }

    pub fn change_selection(&mut self, selected: bool) -> bool {
        let old = self.0;
        if selected {
            self.0 |= Self::SELECTION_STATE_BITS
        } else {
            self.0 &= !Self::SELECTION_STATE_BITS
        }

        old != self.0
    }

    pub fn state(self) -> u32 {
        self.0 & !Self::SELECTION_STATE_BITS
    }

    pub fn change_state(&mut self, state: u32) -> bool {
        let old = self.0;
        self.0 = state & !Self::SELECTION_STATE_BITS;

        old != self.0
    }
}

pub struct Window<C, W, I>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
    I: InputController,
{
    pub canvas: C,
    pub root: W,
    pub input_controller: I,
}

impl<C, W> Window<C, W, DefaultInputController>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub fn new(canvas: C, mut root: W) -> Self {
        root.attach(0, 0);
        Window {
            canvas,
            root,
            input_controller: DefaultInputController::new(),
        }
    }
}

impl<C, W, I> Window<C, W, I>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
    I: InputController,
{
    pub fn with_input_controller<I2>(self, input_controller: I2) -> Window<C, W, I2>
    where
        I2: InputController,
    {
        Window {
            canvas: self.canvas,
            root: self.root,
            input_controller,
        }
    }

    pub fn update(&mut self) {
        let child_count = self.root.children();

        // TODO it might be better to propagate this recursively
        self.root.update();
        for idx in 0..child_count {
            self.root.get_mut_child(idx).update();
        }
    }

    pub fn measure(&mut self) {
        self.root.measure(MeasureSpec {
            width: MeasureConstraint::AtMost(self.canvas.size().width),
            height: MeasureConstraint::AtMost(self.canvas.size().height),
        });
    }

    pub fn arrange(&mut self) {
        self.root.arrange(Position { x: 0, y: 0 });
    }

    pub fn draw(&mut self) -> Result<(), C::Error> {
        self.root.draw(&mut self.canvas)
    }

    pub fn input_event(&mut self, event: InputEvent) {
        self.input_controller.input_event(&mut self.root, event);
    }
}
