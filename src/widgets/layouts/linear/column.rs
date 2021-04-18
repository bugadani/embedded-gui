use object_chain::Chain;

use crate::{
    geometry::{BoundingBox, MeasuredSize, Position},
    widgets::{
        layouts::linear::{
            layout::{LayoutDirection, LinearLayout},
            Cell, CellWeight, NoSpacing,
        },
        Widget,
    },
    MeasureConstraint, MeasureSpec,
};

#[derive(Copy, Clone)]
pub struct Column;

impl LayoutDirection for Column {
    fn main_axis_size(bounds: BoundingBox) -> u32 {
        bounds.size.height
    }

    fn cross_axis_size(bounds: BoundingBox) -> u32 {
        bounds.size.width
    }

    fn create_measured_size(main: u32, cross: u32) -> MeasuredSize {
        MeasuredSize {
            width: cross,
            height: main,
        }
    }

    fn main_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint {
        spec.height
    }

    fn cross_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint {
        spec.width
    }

    fn create_measure_spec(main: MeasureConstraint, cross: MeasureConstraint) -> MeasureSpec {
        MeasureSpec {
            width: cross,
            height: main,
        }
    }

    fn arrange(pos: Position, bb: BoundingBox, spacing: u32) -> Position {
        Position {
            x: pos.x,
            y: pos.y + bb.size.height as i32 + spacing as i32,
        }
    }
}

impl Column {
    pub fn new<W, CW>(widget: Cell<W, CW>) -> LinearLayout<Chain<Cell<W, CW>>, Column, NoSpacing>
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
