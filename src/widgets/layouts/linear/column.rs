use object_chain::Chain;

use crate::{
    geometry::{axis_order::Vertical, BoundingBox},
    widgets::{
        layouts::linear::{
            layout::{LayoutDirection, LinearLayout},
            Cell, CellWeight, ElementSpacing, NoSpacing, NoWeight, WithSpacing,
        },
        Widget,
    },
};

#[derive(Copy, Clone)]
pub struct Column<ES = NoSpacing>(ES)
where
    ES: ElementSpacing;

impl<ES> LayoutDirection for Column<ES>
where
    ES: ElementSpacing,
{
    type AxisOrder = Vertical;

    fn element_spacing(&self) -> u32 {
        self.0.spacing()
    }
}

impl Column {
    pub fn new() -> Self {
        Self(NoSpacing)
    }

    pub fn spacing(self, spacing: u32) -> Column<WithSpacing> {
        Column(WithSpacing(spacing))
    }
}

impl<ES> Column<ES>
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
            parent_index: 0,
            bounds: BoundingBox::default(),
            widgets: Chain::new(widget),
            direction: self,
        }
    }
}
