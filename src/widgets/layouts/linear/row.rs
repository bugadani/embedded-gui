use object_chain::Chain;

use crate::{
    widgets::{
        layouts::linear::{
            layout::{LayoutDirection, LinearLayout},
            Cell, CellWeight, NoSpacing,
        },
        Widget,
    },
    BoundingBox, MeasureConstraint, MeasureSpec, MeasuredSize, Position,
};

#[derive(Copy, Clone)]
pub struct Row;

impl LayoutDirection for Row {
    fn main_axis_size(bounds: BoundingBox) -> u32 {
        bounds.size.width
    }

    fn cross_axis_size(bounds: BoundingBox) -> u32 {
        bounds.size.height
    }

    fn create_measured_size(main: u32, cross: u32) -> MeasuredSize {
        MeasuredSize {
            width: main,
            height: cross,
        }
    }

    fn main_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint {
        spec.width
    }

    fn cross_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint {
        spec.height
    }

    fn create_measure_spec(main: MeasureConstraint, cross: MeasureConstraint) -> MeasureSpec {
        MeasureSpec {
            width: main,
            height: cross,
        }
    }

    fn arrange(pos: Position, bb: BoundingBox, spacing: u32) -> Position {
        Position {
            x: pos.x + bb.size.width as i32 + spacing as i32,
            y: pos.y,
        }
    }
}

impl Row {
    pub fn new<W, CW>(widget: Cell<W, CW>) -> LinearLayout<Chain<Cell<W, CW>>, Row, NoSpacing>
    where
        W: Widget,
        CW: CellWeight,
    {
        LinearLayout {
            parent_index: 0,
            bounds: BoundingBox::default(),
            widgets: Chain::new(widget),
            spacing: NoSpacing,
            direction: Self,
        }
    }
}
