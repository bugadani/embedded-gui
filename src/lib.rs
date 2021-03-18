//#![no_std]

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

pub struct Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub canvas: C,
    pub root: W,
    pub selected_index: Option<usize>,
    pub temp_selected_index: Option<usize>,
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
            temp_selected_index: None,
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

    fn change_selected_widget(&mut self, selected: Option<usize>) {
        if self.selected_index != selected {
            if self.selected_index != self.temp_selected_index {
                self.selected_index
                    .map(|idx| self.get_mut_widget(idx).change_selection(false));
            }
            selected.map(|idx| self.get_mut_widget(idx).change_selection(true));

            self.selected_index = selected;
        }
    }

    fn change_temp_selected_widget(&mut self, selected: Option<usize>) {
        if self.temp_selected_index != selected {
            if self.temp_selected_index != self.selected_index {
                self.temp_selected_index
                    .map(|idx| self.get_mut_widget(idx).change_selection(false));
            }
            selected.map(|idx| self.get_mut_widget(idx).change_selection(true));

            self.temp_selected_index = selected;
        }
    }

    fn get_mut_widget(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.root
        } else {
            self.root.get_mut_child(idx - 1)
        }
    }

    fn get_widget(&mut self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.root
        } else {
            self.root.get_child(idx - 1)
        }
    }

    pub fn input_event(&mut self, event: InputEvent) {
        // TODO: button's click shouldn't grab selection - inputcontext should have a method to grab
        // a temp selection.
        // Click out of a widget should clear selection.
        let selected_idx = match event.selection_modifier() {
            SelectionModifier::None => self.selected_index,
            SelectionModifier::GrabSelection(position) => {
                let res = self.hit_test(position);
                self.change_selected_widget(res);

                res
            }
            SelectionModifier::TempSelection(position) => {
                let res = self.hit_test(position);
                self.change_temp_selected_widget(res);

                res
            }
        };

        if let Some(idx) = selected_idx {
            // TODO: this needs to be redone.
            let mut input_ctxt = InputCtxt {
                child_count: self.root.children(),
                selected_index: selected_idx,
                select_next: false,
                clear_selection: false,
            };

            let widget = self.get_mut_widget(idx);
            widget.handle_input(&mut input_ctxt, event);

            if input_ctxt.clear_selection {
                self.change_selected_widget(None);
            } else if input_ctxt.select_next {
                let old_selected = self.selected_index;

                // It's possible the tree doesn't contain selectable widgets at all.
                loop {
                    let new_selected = match self.selected_index {
                        None => 0,
                        Some(idx) if idx == self.root.children() => {
                            if old_selected == None {
                                self.change_selected_widget(None);
                                break;
                            }
                            0
                        }
                        Some(idx) => {
                            if Some(idx + 1) == old_selected {
                                self.change_selected_widget(None);
                                break;
                            }
                            idx + 1
                        }
                    };

                    if self.get_widget(new_selected).is_selectable() {
                        self.change_selected_widget(Some(new_selected));
                        break;
                    }
                }
            }
        }
    }

    fn hit_test(&self, position: Position) -> Option<usize> {
        self.root.hit_test(position)
    }
}

pub struct InputCtxt {
    pub child_count: usize,
    pub selected_index: Option<usize>,
    pub select_next: bool,
    pub clear_selection: bool,
}

impl InputCtxt {
    pub fn select_next_widget(&mut self) {
        self.select_next = true;
    }

    pub fn clear_selection(&mut self) {
        self.clear_selection = true;
    }
}
