use object_chain::Chain;

use crate::{
    geometry::{axis_order::Vertical, BoundingBox},
    widgets::{
        layouts::linear::{
            layout::{LayoutDirection, LinearLayout},
            Cell, CellWeight, NoSpacing, NoWeight,
        },
        Widget,
    },
};

#[derive(Copy, Clone)]
pub struct Column;

impl LayoutDirection for Column {
    type AxisOrder = Vertical;
}

impl Column {
    pub fn new<W>(widget: W) -> LinearLayout<Chain<Cell<W, NoWeight>>, Column, NoSpacing>
    where
        W: Widget,
    {
        Self::new_cell(Cell::new(widget))
    }

    pub fn new_cell<W, CW>(
        widget: Cell<W, CW>,
    ) -> LinearLayout<Chain<Cell<W, CW>>, Column, NoSpacing>
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
