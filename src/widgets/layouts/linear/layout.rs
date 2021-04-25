use object_chain::{ChainElement, Link};

use crate::{
    geometry::{
        measurement::{MeasureConstraint, MeasureSpec},
        BoundingBox, MeasuredSize, Position,
    },
    input::event::InputEvent,
    state::WidgetState,
    widgets::{
        layouts::linear::{
            Cell, CellWeight, ElementSpacing, LinearLayoutChainElement, NoSpacing, WithSpacing,
        },
        ParentHolder, UpdateHandler, Widget, WidgetStateHolder,
    },
    Canvas, WidgetRenderer,
};

pub trait LayoutDirection: Copy {
    fn main_axis_size(bounds: BoundingBox) -> u32;
    fn cross_axis_size(bounds: BoundingBox) -> u32;
    fn create_measured_size(main: u32, cross: u32) -> MeasuredSize;
    fn main_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint;
    fn cross_axis_measure_spec(spec: MeasureSpec) -> MeasureConstraint;
    fn create_measure_spec(main: MeasureConstraint, cross: MeasureConstraint) -> MeasureSpec;
    fn arrange(pos: Position, bb: BoundingBox, spacing: u32) -> Position;
}

pub struct LinearLayout<CE, L, ES = NoSpacing> {
    pub parent_index: usize,
    pub bounds: BoundingBox,
    pub widgets: CE,
    pub spacing: ES,
    pub direction: L,
}

impl<CE, L, ES> LinearLayout<CE, L, ES>
where
    CE: LinearLayoutChainElement + ChainElement,
    ES: ElementSpacing,
{
    pub fn add<W, CW>(self, widget: Cell<W, CW>) -> LinearLayout<Link<Cell<W, CW>, CE>, L, ES>
    where
        W: Widget,
        CW: CellWeight,
    {
        LinearLayout {
            parent_index: self.parent_index,
            bounds: self.bounds,
            widgets: self.widgets.append(widget),
            spacing: self.spacing,
            direction: self.direction,
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

impl<CE, L> LinearLayout<CE, L, NoSpacing> {
    pub fn spacing(self, spacing: u32) -> LinearLayout<CE, L, WithSpacing> {
        LinearLayout {
            parent_index: self.parent_index,
            bounds: self.bounds,
            widgets: self.widgets,
            spacing: WithSpacing(spacing),
            direction: self.direction,
        }
    }
}

impl<CE, L, ES> Widget for LinearLayout<CE, L, ES>
where
    CE: LinearLayoutChainElement + ChainElement,
    ES: ElementSpacing,
    L: LayoutDirection,
{
    fn attach(&mut self, parent: usize, index: usize) {
        self.set_parent(parent);

        let mut children = 0;

        for i in 0..self.widgets.len() {
            let widget = self.widgets.at_mut(i).widget_mut();

            widget.attach(index, children + i + 1);
            children += widget.children();
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let count = self.widgets.len();
        let mut total_fixed_main_axis_size = 0;
        let mut max_cross = 0;

        let max_main_axis_size = match L::main_axis_measure_spec(measure_spec) {
            MeasureConstraint::AtMost(max) | MeasureConstraint::Exactly(max) => max,
            MeasureConstraint::Unspecified => {
                // Weight makes no sense here as we have "infinite" space.
                for i in 0..count {
                    let widget = self.widgets.at_mut(i).widget_mut();

                    widget.measure(measure_spec);
                    total_fixed_main_axis_size += L::main_axis_size(widget.bounding_box());
                    max_cross = max_cross.max(L::cross_axis_size(widget.bounding_box()));
                }

                total_fixed_main_axis_size += (count as u32 - 1) * self.spacing.spacing();

                // TODO this is almost the same as the bottom of the function. Should deduplicate.
                self.set_measured_size(L::create_measured_size(
                    total_fixed_main_axis_size,
                    L::cross_axis_measure_spec(measure_spec).apply_to_measured(max_cross),
                ));

                return;
            }
        };

        let mut total_weight = 0;

        // Count the height of the widgets that don't have a weight
        for i in 0..count {
            let cell = self.widgets.at_mut(i);
            let weight = cell.weight();
            if weight == 0 {
                let spec = L::create_measure_spec(
                    MeasureConstraint::AtMost(max_main_axis_size - total_fixed_main_axis_size),
                    L::cross_axis_measure_spec(measure_spec),
                );

                let widget = cell.widget_mut();
                widget.measure(spec);
                total_fixed_main_axis_size += L::main_axis_size(widget.bounding_box());
                max_cross = max_cross.max(L::cross_axis_size(widget.bounding_box()));
            } else {
                total_weight += weight;
            }
        }

        // We don't want to take space away from non-weighted widgets,
        // so add spacing after first pass.
        total_fixed_main_axis_size += (count as u32 - 1) * self.spacing.spacing();

        // Divide the rest of the space among the weighted widgets
        if total_weight != 0 {
            let remaining_space = max_main_axis_size - total_fixed_main_axis_size;
            let height_per_weight_unit = remaining_space / total_weight;
            // in case we have some stray pixels, divide them up evenly
            let mut remainder = remaining_space % total_weight;

            for i in 0..count {
                let cell = self.widgets.at_mut(i);
                let weight = cell.weight();
                if weight != 0 {
                    let rem = if remainder > 0 {
                        remainder.min(weight)
                    } else {
                        0
                    };

                    remainder -= rem;

                    let spec = L::create_measure_spec(
                        MeasureConstraint::Exactly(height_per_weight_unit * weight + rem),
                        L::cross_axis_measure_spec(measure_spec),
                    );

                    let widget = cell.widget_mut();
                    widget.measure(spec);

                    max_cross = max_cross.max(L::cross_axis_size(widget.bounding_box()));
                }
            }
        }

        self.set_measured_size(L::create_measured_size(
            if total_weight == 0 {
                total_fixed_main_axis_size
            } else {
                max_main_axis_size
            },
            L::cross_axis_measure_spec(measure_spec).apply_to_measured(max_cross),
        ))
    }

    fn arrange(&mut self, position: Position) {
        self.bounding_box_mut().position = position;

        self.widgets
            .arrange(position, self.direction, self.spacing.spacing());
    }

    fn children(&self) -> usize {
        self.widgets.count_widgets()
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
        self.widgets.test_input(event).map(|idx| idx + 1)
    }
}

impl<CE, L, ES> ParentHolder for LinearLayout<CE, L, ES> {
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}

impl<CE, L, ES> WidgetStateHolder for LinearLayout<CE, L, ES>
where
    CE: LinearLayoutChainElement + ChainElement,
{
    fn on_state_changed(&mut self, state: WidgetState) {
        self.widgets.on_state_changed(state);
    }
}

impl<CE, L, ES> UpdateHandler for LinearLayout<CE, L, ES> {}

impl<C, CE, L, ES> WidgetRenderer<C> for LinearLayout<CE, L, ES>
where
    CE: LinearLayoutChainElement + ChainElement + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.widgets.draw(canvas)
    }
}
