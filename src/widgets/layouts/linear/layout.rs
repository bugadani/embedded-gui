use object_chain::{Chain, ChainElement, Link};

use crate::{
    geometry::{
        measurement::{MeasureConstraint, MeasureSpec},
        BoundingBox, Position,
    },
    input::event::InputEvent,
    state::WidgetState,
    widgets::{
        layouts::linear::{
            private::{LayoutDirection, LinearLayoutChainElement},
            Cell, NoWeight, Weight,
        },
        Widget,
    },
    Canvas, WidgetRenderer,
};

/// Common implementation of linear layouts.
///
/// ## Cell measurement
///
/// The measurement process consists of two phases. At first, cells without weight are measured.
/// Then, the rest of the available space is partitioned according to the weights of the remaining
/// cells. The more weight a cell has, the more space it will take up.
///
/// ### Example:
///
/// We have a layout with three cells:
///  - A cell with weight 1.
///  - A cell without weight, which contains a widget that is 40 px high.
///  - A cell with weight 2.
///
/// The layout has 160 px height.
///
/// 1. At first, the second cell is measured. It needs 40 px for its widget, which leaves 120 px of
/// unused space.
/// 2. Next, the remaining space is divided into 3 (the sum of weights), and assigned to the rest of
/// the cells. Each cell receives space according to its weight.
///
/// The final layout will look like this:
///  - The first cell will take up 40 px because of weight 1.
///  - The second cell will take up 40 px because it has a fixed height of 40.
///  - The third cell will take up 80 px because of weight 2.
///
pub struct LinearLayout<CE, L> {
    pub bounds: BoundingBox,
    pub widgets: CE,
    pub direction: L,
}

impl<CE, L> LinearLayout<CE, L>
where
    CE: LinearLayoutChainElement + ChainElement,
{
    /// Appends a cell to the end of the layout.
    ///
    /// Initially, a cell has no weight. Call `weight` to specify the weight of the most recently
    /// added cell.
    pub fn add<W>(self, widget: W) -> LinearLayout<Link<Cell<W, NoWeight>, CE>, L>
    where
        W: Widget,
    {
        LinearLayout {
            bounds: self.bounds,
            widgets: self.widgets.append(Cell::new(widget)),
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

impl<W, CE, L> LinearLayout<Link<Cell<W, NoWeight>, CE>, L>
where
    W: Widget,
    CE: LinearLayoutChainElement + ChainElement,
{
    /// Sets the weight of the most recently appended cell.
    ///
    /// Weight specifies the relative size of the cell in the second phase of the layout process.
    /// See [`LinearLayout#cell-measurement`] for more information.
    pub fn weight(self, weight: u32) -> LinearLayout<Link<Cell<W, Weight>, CE>, L> {
        LinearLayout {
            bounds: self.bounds,
            widgets: Link {
                object: self.widgets.object.weight(weight),
                parent: self.widgets.parent,
            },
            direction: self.direction,
        }
    }
}

impl<W, L> LinearLayout<Chain<Cell<W, NoWeight>>, L>
where
    W: Widget,
{
    /// Sets the weight of the most recently appended cell.
    ///
    /// Weight specifies the relative size of the cell in the second phase of the layout process.
    /// See [`LinearLayout#cell-measurement`] for more information.
    pub fn weight(self, weight: u32) -> LinearLayout<Chain<Cell<W, Weight>>, L> {
        LinearLayout {
            bounds: self.bounds,
            widgets: Chain {
                object: self.widgets.object.weight(weight),
            },
            direction: self.direction,
        }
    }
}

impl<CE, L> Widget for LinearLayout<CE, L>
where
    CE: LinearLayoutChainElement + ChainElement,
    L: LayoutDirection,
{
    fn attach(&mut self, parent: usize, index: usize) {
        debug_assert!(index == 0 || parent != index);
        let mut children = index;

        for i in 0..self.widgets.len() {
            let widget = self.widgets.at_mut(i).widget_mut();

            widget.attach(parent, children + i);
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

                total_fixed_main_axis_size += (count as u32 - 1) * self.direction.element_spacing();

                // TODO this is almost the same as the bottom of the function. Should deduplicate.
                self.bounds.size = L::create_measured_size(
                    total_fixed_main_axis_size,
                    L::cross_axis_measure_spec(measure_spec).apply_to_measured(max_cross),
                );

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
        total_fixed_main_axis_size += (count as u32 - 1) * self.direction.element_spacing();

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

        self.bounds.size = L::create_measured_size(
            if total_weight == 0 {
                total_fixed_main_axis_size
            } else {
                max_main_axis_size
            },
            L::cross_axis_measure_spec(measure_spec).apply_to_measured(max_cross),
        );
    }

    fn arrange(&mut self, position: Position) {
        self.bounds.position = position;

        self.widgets
            .arrange(position, self.direction, self.direction.element_spacing());
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

    fn parent_index(&self) -> usize {
        self.widgets.at(0).widget().parent_index()
    }

    fn set_parent(&mut self, _index: usize) {}

    fn update(&mut self) {
        self.widgets.update();
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.widgets.test_input(event).map(|idx| idx + 1)
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.widgets.on_state_changed(state);
    }

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<C, CE, L> WidgetRenderer<C> for LinearLayout<CE, L>
where
    CE: LinearLayoutChainElement + ChainElement + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&mut self, canvas: &mut C) -> Result<(), C::Error> {
        self.widgets.draw(canvas)
    }
}
