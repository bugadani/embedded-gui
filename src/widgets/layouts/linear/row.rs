use core::marker::PhantomData;

use object_chain::{Chain, ChainElement, Link};

use crate::{
    input::event::InputEvent,
    widgets::{
        layouts::linear::{Cell, LinearLayoutChainElement},
        ParentHolder, Widget, WidgetStateHolder,
    },
    BoundingBox, Canvas, MeasureConstraint, MeasureSpec, MeasuredSize, Position, WidgetRenderer,
};

pub struct Row<C, CE> {
    parent_index: Option<usize>,
    bounds: BoundingBox,
    total_weight: u32,
    widgets: CE,
    _marker: PhantomData<C>,
}

impl<C> Row<C, ()>
where
    C: Canvas,
{
    pub fn new<W>(widget: Cell<W>) -> Row<C, Chain<Cell<W>>>
    where
        W: Widget,
    {
        Row {
            parent_index: None,
            bounds: BoundingBox::default(),
            total_weight: widget.weight.unwrap_or(0),
            widgets: Chain::new(widget),
            _marker: PhantomData,
        }
    }
}

impl<C, CE> Row<C, CE>
where
    C: Canvas,
    CE: LinearLayoutChainElement<C> + ChainElement,
{
    pub fn add<W>(self, widget: Cell<W>) -> Row<C, Link<Cell<W>, CE>>
    where
        W: Widget,
    {
        Row {
            parent_index: self.parent_index,
            bounds: self.bounds,
            total_weight: self.total_weight + widget.weight.unwrap_or(0),
            widgets: self.widgets.append(widget),
            _marker: PhantomData,
        }
    }

    fn locate(&self, mut idx: usize) -> Option<(usize, usize)> {
        let children = self.widgets.len();

        for i in 0..children {
            let child = self.widgets.at(i).widget();
            let grandchildren = child.children();
            if idx <= grandchildren {
                return Some((i, idx));
            }

            idx -= grandchildren + 1;
        }

        None
    }
}

impl<C, CE> Widget for Row<C, CE>
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
        let max_width = match measure_spec.width {
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
        let mut fixed_widths = 0;
        let mut max_height = 0;

        // Count the height of the widgets that don't have a weight
        for i in 0..count {
            let cell = self.widgets.at_mut(i);
            if cell.weight().is_none() {
                let spec = MeasureSpec {
                    width: MeasureConstraint::AtMost(max_width - fixed_widths),
                    height: measure_spec.height,
                };

                let widget = cell.widget_mut();
                widget.measure(spec);
                fixed_widths += widget.bounding_box().size.width;
                max_height = max_height.max(widget.bounding_box().size.height);
            }
        }

        // Divide the rest of the space among the weighted widgets
        if self.total_weight != 0 {
            let remaining_space = max_width - fixed_widths;
            let width_per_weight_unit = remaining_space / self.total_weight;
            // in case we have some stray pixels, divide them up evenly
            let remainder = remaining_space % self.total_weight;

            for i in 0..count {
                let cell = self.widgets.at_mut(i);
                if let Some(weight) = cell.weight() {
                    let spec = MeasureSpec {
                        width: MeasureConstraint::Exactly(
                            width_per_weight_unit * weight + ((i as u32) < remainder) as u32,
                        ),
                        height: measure_spec.height,
                    };

                    let widget = cell.widget_mut();
                    widget.measure(spec);
                    max_height = max_height.max(widget.bounding_box().size.height);
                }
            }
        }

        self.set_measured_size(MeasuredSize {
            width: if self.total_weight == 0 {
                fixed_widths
            } else {
                max_width
            },
            height: measure_spec.height.apply_to_measured(max_height),
        })
    }

    fn arrange(&mut self, mut position: Position) {
        self.bounding_box_mut().position = position;

        let count = self.widgets.len();
        for i in 0..count {
            let widget = self.widgets.at_mut(i).widget_mut();

            widget.arrange(position);

            let width = widget.bounding_box().size.width;
            position.x += width as i32;
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
        let (child, grandchild) = self.locate(idx).unwrap();

        let widget = self.widgets.at(child).widget();
        if grandchild == 0 {
            widget
        } else {
            widget.get_child(grandchild - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        let (child, grandchild) = self.locate(idx).unwrap();

        let widget = self.widgets.at_mut(child).widget_mut();
        if grandchild == 0 {
            widget
        } else {
            widget.get_mut_child(grandchild - 1)
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

impl<C, CE> ParentHolder for Row<C, CE>
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

impl<C, CE> WidgetStateHolder for Row<C, CE>
where
    C: Canvas,
{
    fn change_state(&mut self, _state: u32) {}

    fn change_selection(&mut self, _state: bool) {}
}

impl<C, CE> WidgetRenderer<C> for Row<C, CE>
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
