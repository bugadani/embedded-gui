use object_chain::Chain;

use crate::{
    geometry::{axis_order::Horizontal, BoundingBox},
    widgets::{
        layouts::linear::{
            layout::{LayoutDirection, LinearLayout},
            Cell, CellWeight, NoSpacing, NoWeight,
        },
        Widget,
    },
};

#[derive(Copy, Clone)]
pub struct Row;

impl LayoutDirection for Row {
    type AxisOrder = Horizontal;
}

impl Row {
    pub fn new<W>(widget: W) -> LinearLayout<Chain<Cell<W, NoWeight>>, Row, NoSpacing>
    where
        W: Widget,
    {
        Self::new_cell(Cell::new(widget))
    }

    pub fn new_cell<W, CW>(widget: Cell<W, CW>) -> LinearLayout<Chain<Cell<W, CW>>, Row, NoSpacing>
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
