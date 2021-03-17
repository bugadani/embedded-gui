#![no_std]

pub mod data;
pub mod input;
pub mod widgets;

use input::SelectionModifier;

use crate::{
    data::WidgetData,
    input::InputEvent,
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
    pub fn hit_test(&self, position: Position) -> bool {
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
}

#[derive(Copy, Clone, Debug)]
pub struct MeasureSpec {
    width: MeasureConstraint,
    height: MeasureConstraint,
}

pub struct Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub canvas: C,
    pub root: W,
    pub selected_index: Option<usize>,
}

impl<C, W> Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub fn new(canvas: C, root: W) -> Self {
        Self {
            canvas,
            root,
            selected_index: None,
        }
    }

    pub fn update(&mut self) {
        let child_count = self.root.children();

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
        let selected_idx = match event.selection_modifier() {
            SelectionModifier::None => self.selected_index,
            SelectionModifier::GrabSelection(position) => {
                let res = self.hit_test(position);
                self.selected_index = res;

                res
            }
            SelectionModifier::TempSelection(position) => self.hit_test(position),
        };

        if let Some(idx) = selected_idx {
            let mut input_ctxt = InputCtxt {
                child_count: self.root.children(),
                selected_index: self.selected_index,
            };
            let widget = if idx == 0 {
                &mut self.root
            } else {
                self.root.get_mut_child(idx)
            };

            widget.handle_input(&mut input_ctxt, event);

            self.selected_index = input_ctxt.selected_index;
        }
    }

    fn hit_test(&self, position: Position) -> Option<usize> {
        self.root.hit_test(position)
    }
}

pub struct InputCtxt {
    pub child_count: usize,
    pub selected_index: Option<usize>,
}

impl InputCtxt {
    pub fn select_next_widget(&mut self) {
        match self.selected_index.as_mut() {
            Some(index) => {
                if *index == self.child_count {
                    *index = 0;
                } else {
                    *index += 1;
                }
            }
            None => self.selected_index = Some(0),
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_index = None;
    }
}
