use core::marker::PhantomData;

use object_chain::{Chain, ChainElement, Link};

use crate::{
    input::event::InputEvent,
    widgets::{
        layouts::linear::{Cell, LinearLayoutChainElement},
        ParentHolder, Widget, WidgetStateHolder,
    },
    BoundingBox, Canvas, MeasureConstraint, MeasureSpec, Position, WidgetRenderer,
};

pub struct Column<C, CE> {
    parent_index: Option<usize>,
    bounds: BoundingBox,
    total_weight: u32,
    widgets: CE,
    _marker: PhantomData<C>,
}

impl<C> Column<C, ()>
where
    C: Canvas,
{
    pub fn new<W>(widget: Cell<W>) -> Column<C, Chain<Cell<W>>>
    where
        W: Widget,
    {
        Column {
            parent_index: None,
            bounds: BoundingBox::default(),
            total_weight: widget.weight.unwrap_or(0),
            widgets: Chain::new(widget),
            _marker: PhantomData,
        }
    }
}

impl<C, CE> Column<C, CE>
where
    C: Canvas,
    CE: LinearLayoutChainElement<C> + ChainElement,
{
    pub fn add<W>(self, widget: Cell<W>) -> Column<C, Link<Cell<W>, CE>>
    where
        W: Widget,
    {
        Column {
            parent_index: self.parent_index,
            bounds: self.bounds,
            total_weight: self.total_weight + widget.weight.unwrap_or(0),
            widgets: self.widgets.append(widget),
            _marker: PhantomData,
        }
    }
}

impl<C, CE> Widget for Column<C, CE>
where
    C: Canvas,
    CE: LinearLayoutChainElement<C> + ChainElement,
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let max_height = match measure_spec.height {
            MeasureConstraint::AtMost(max) | MeasureConstraint::Exactly(max) => max,
            MeasureConstraint::Unspecified => {
                // We can do whatever
                let count = self.widgets.len();
                for i in 0..count {
                    self.widgets.at_mut(i).widget_mut().measure(measure_spec);
                }
                return;
            }
        };

        let count = self.widgets.len();
        let mut fixed_heights = 0;
        // Count the height of the widgets that don't have a weight
        for i in 0..count {
            let cell = self.widgets.at_mut(i);
            if cell.weight().is_none() {
                let spec = MeasureSpec {
                    width: measure_spec.width,
                    height: MeasureConstraint::AtMost(max_height - fixed_heights),
                };

                let widget = cell.widget_mut();
                widget.measure(spec);
                fixed_heights += widget.bounding_box().size.height;
            }
        }

        // Divide the rest of the space among the weighted widgets
        if self.total_weight != 0 {
            let remaining_space = max_height - fixed_heights;
            let height_per_weight_unit = remaining_space / self.total_weight;
            // in case we have some stray pixels, divide them up evenly
            let remainder = remaining_space % self.total_weight;

            for i in 0..count {
                let cell = self.widgets.at_mut(i);
                if let Some(weight) = cell.weight() {
                    let spec = MeasureSpec {
                        width: measure_spec.width,
                        height: MeasureConstraint::Exactly(
                            height_per_weight_unit * weight + ((i as u32) < remainder) as u32,
                        ),
                    };

                    let widget = cell.widget_mut();
                    widget.measure(spec);
                    fixed_heights += widget.bounding_box().size.height;
                }
            }
        }
    }

    fn children(&self) -> usize {
        let count = self.widgets.len();
        let mut children = count;
        for i in 0..count {
            children += self.widgets.at(i).widget().children();
        }

        children
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            self
        } else {
            let mut index = idx - 1;
            let children = self.children();
            let mut child_idx = None;
            for i in 0..children {
                let child = self.widgets.at(i).widget();
                let grandchildren = child.children();
                if index < grandchildren {
                    child_idx = Some(i);
                    break;
                }

                index -= grandchildren - 1;
            }

            let widget = self.widgets.at(child_idx.unwrap()).widget();
            if index == 0 {
                widget
            } else {
                widget.get_child(index - 1)
            }
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            self
        } else {
            let mut index = idx - 1;
            let children = self.children();
            let mut child_idx = None;
            for i in 0..children {
                let child = self.widgets.at(i).widget();
                let grandchildren = child.children();
                if index <= grandchildren {
                    child_idx = Some(i);
                    break;
                }

                index -= grandchildren;
            }

            let widget = self.widgets.at_mut(child_idx.unwrap()).widget_mut();
            if index == 0 {
                widget
            } else {
                widget.get_mut_child(index - 1)
            }
        }
    }

    fn arrange(&mut self, mut position: Position) {
        self.bounding_box_mut().position = position;

        let count = self.widgets.len();
        for i in 0..count {
            let widget = self.widgets.at_mut(i).widget_mut();

            widget.arrange(position);

            let height = widget.bounding_box().size.height;
            position.y += height as i32;
        }
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        let mut index = 1;

        let count = self.widgets.len();
        for i in 0..count {
            let widget = self.widgets.at_mut(i).widget_mut();

            if let Some(idx) = widget.test_input(event) {
                return Some(idx + index);
            }

            index += widget.children() + 1;
        }

        None
    }
}

impl<C, CE> ParentHolder for Column<C, CE>
where
    C: Canvas,
{
    fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    fn set_parent(&mut self, index: Option<usize>) {
        self.parent_index = index;
    }
}

impl<C, CE> WidgetStateHolder for Column<C, CE>
where
    C: Canvas,
{
    fn change_state(&mut self, _state: u32) {}

    fn change_selection(&mut self, _state: bool) {}
}

impl<C, CE> WidgetRenderer<C> for Column<C, CE>
where
    CE: LinearLayoutChainElement<C> + ChainElement,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        let count = self.widgets.len();
        for i in 0..count {
            self.widgets.at(i).draw(canvas)?;
        }
        Ok(())
    }
}
