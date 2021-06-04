use object_chain::Chain;

use crate::{
    geometry::{axis_order::Horizontal, BoundingBox},
    widgets::{
        layouts::linear::{
            layout::{LayoutDirection, LinearLayout},
            Cell, CellWeight, ElementSpacing, NoSpacing, NoWeight, WithSpacing,
        },
        Widget,
    },
};

#[derive(Copy, Clone)]
pub struct Row<ES = NoSpacing>(ES)
where
    ES: ElementSpacing;

impl<ES> LayoutDirection for Row<ES>
where
    ES: ElementSpacing,
{
    type AxisOrder = Horizontal;

    fn element_spacing(&self) -> u32 {
        self.0.spacing()
    }
}

impl Row {
    pub fn new() -> Self {
        Self(NoSpacing)
    }

    pub fn spacing(self, spacing: u32) -> Row<WithSpacing> {
        Row(WithSpacing(spacing))
    }
}

impl<ES> Row<ES>
where
    ES: ElementSpacing,
{
    pub fn add<W>(self, widget: W) -> LinearLayout<Chain<Cell<W, NoWeight>>, Self>
    where
        W: Widget,
    {
        self.add_cell(Cell::new(widget))
    }

    pub fn add_cell<W, CW>(self, widget: Cell<W, CW>) -> LinearLayout<Chain<Cell<W, CW>>, Self>
    where
        W: Widget,
        CW: CellWeight,
    {
        LinearLayout {
            bounds: BoundingBox::default(),
            widgets: Chain::new(widget),
            direction: self,
        }
    }
}
