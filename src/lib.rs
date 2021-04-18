#![no_std]

pub mod data;
pub mod geometry;
pub mod input;
pub mod widgets;

use crate::{
    geometry::{measurement::MeasureSpec, MeasuredSize, Position},
    input::{
        controller::{DefaultInputController, InputController},
        event::InputEvent,
    },
    widgets::Widget,
};

pub trait WidgetRenderer<C: Canvas> {
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error>;
}

pub trait Canvas {
    type Error;

    fn size(&self) -> MeasuredSize;
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
        self.root
            .measure(MeasureSpec::from_measured_at_most(self.canvas.size()));
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
