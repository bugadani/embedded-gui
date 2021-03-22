use object_chain::{Chain, ChainElement, Link};

use crate::{
    input::event::InputEvent,
    widgets::{
        layouts::linear::{Cell, CellWeight, LinearLayoutChainElement},
        ParentHolder, UpdateHandler, Widget, WidgetStateHolder,
    },
    BoundingBox, Canvas, MeasureConstraint, MeasureSpec, MeasuredSize, Position, WidgetRenderer,
};

pub struct Row<CE> {
    parent_index: usize,
    bounds: BoundingBox,
    widgets: CE,
}

impl Row<()> {
    pub fn new<W, CW>(widget: Cell<W, CW>) -> Row<Chain<Cell<W, CW>>>
    where
        W: Widget,
        CW: CellWeight,
    {
        Row {
            parent_index: 0,
            bounds: BoundingBox::default(),
            widgets: Chain::new(widget),
        }
    }
}

impl<CE> Row<CE>
where
    CE: LinearLayoutChainElement + ChainElement,
{
    pub fn add<W, CW>(self, widget: Cell<W, CW>) -> Row<Link<Cell<W, CW>, CE>>
    where
        W: Widget,
        CW: CellWeight,
    {
        Row {
            parent_index: self.parent_index,
            bounds: self.bounds,
            widgets: self.widgets.append(widget),
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

impl<CE> Widget for Row<CE>
where
    CE: LinearLayoutChainElement + ChainElement,
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
        let mut total_weight = 0;

        // Count the width of the widgets that don't have a weight
        for i in 0..count {
            let cell = self.widgets.at_mut(i);
            let weight = cell.weight();
            if weight == 0 {
                let spec = MeasureSpec {
                    width: MeasureConstraint::AtMost(max_width - fixed_widths),
                    height: measure_spec.height,
                };

                let widget = cell.widget_mut();
                widget.measure(spec);
                fixed_widths += widget.bounding_box().size.width;
                max_height = max_height.max(widget.bounding_box().size.height);
            } else {
                total_weight += weight;
            }
        }

        // Divide the rest of the space among the weighted widgets
        if total_weight != 0 {
            let remaining_space = max_width - fixed_widths;
            let width_per_weight_unit = remaining_space / total_weight;
            // in case we have some stray pixels, divide them up evenly
            let remainder = remaining_space % total_weight;

            for i in 0..count {
                let cell = self.widgets.at_mut(i);
                let weight = cell.weight();
                if weight != 0 {
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
            width: if total_weight == 0 {
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

impl<CE> ParentHolder for Row<CE> {
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}

impl<CE> WidgetStateHolder for Row<CE> {
    fn change_state(&mut self, _state: u32) {}

    fn change_selection(&mut self, _state: bool) {}
}

impl<CE> UpdateHandler for Row<CE> {}

impl<C, CE> WidgetRenderer<C> for Row<CE>
where
    CE: LinearLayoutChainElement + ChainElement + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.widgets.draw(canvas)
    }
}
