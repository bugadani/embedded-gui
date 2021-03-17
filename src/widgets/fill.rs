use crate::{
    input::InputEvent, widgets::Widget, BoundingBox, InputCtxt, MeasureConstraint, MeasureSpec,
    MeasuredSize, Position,
};

pub trait FillDirection {
    fn measure<W: Widget>(widget: &mut W, child_size: MeasuredSize, measure_spec: MeasureSpec);
}
pub struct Horizontal;
pub struct Vertical;
pub struct HorizontalAndVertical;

fn stretch_with_constraint(size: u32, constraint: MeasureConstraint) -> u32 {
    match constraint {
        MeasureConstraint::AtMost(constraint) => constraint,
        MeasureConstraint::Exactly(constraint) => constraint,
        MeasureConstraint::Unspecified => size,
    }
}

impl FillDirection for Horizontal {
    fn measure<W: Widget>(widget: &mut W, child_size: MeasuredSize, measure_spec: MeasureSpec) {
        let height = child_size.height;
        let width = stretch_with_constraint(child_size.width, measure_spec.width);

        widget.set_measured_size(MeasuredSize { width, height })
    }
}

impl FillDirection for Vertical {
    fn measure<W: Widget>(widget: &mut W, child_size: MeasuredSize, measure_spec: MeasureSpec) {
        let height = stretch_with_constraint(child_size.height, measure_spec.height);
        let width = child_size.width;

        widget.set_measured_size(MeasuredSize { width, height })
    }
}

impl FillDirection for HorizontalAndVertical {
    fn measure<W: Widget>(widget: &mut W, child_size: MeasuredSize, measure_spec: MeasureSpec) {
        let width = stretch_with_constraint(child_size.width, measure_spec.width);
        let height = stretch_with_constraint(child_size.height, measure_spec.height);

        widget.set_measured_size(MeasuredSize { width, height })
    }
}

pub struct FillParent<W, FD>
where
    FD: FillDirection,
{
    pub inner: W,
    pub direction: FD,
    pub bounds: BoundingBox,
}

impl<W> FillParent<W, Horizontal>
where
    W: Widget,
{
    pub fn horizontal(inner: W) -> FillParent<W, Horizontal> {
        FillParent {
            inner,
            direction: Horizontal,
            bounds: BoundingBox::default(),
        }
    }

    pub fn vertical(inner: W) -> FillParent<W, Vertical> {
        FillParent {
            inner,
            direction: Vertical,
            bounds: BoundingBox::default(),
        }
    }

    pub fn both(inner: W) -> FillParent<W, HorizontalAndVertical> {
        FillParent {
            inner,
            direction: HorizontalAndVertical,
            bounds: BoundingBox::default(),
        }
    }
}

impl<W, D> Widget for FillParent<W, D>
where
    W: Widget,
    D: FillDirection,
{
    fn arrange(&mut self, position: Position) {
        self.bounds.position = position;
        self.inner.arrange(position);
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec);

        D::measure(self, self.inner.bounding_box().size, measure_spec);
    }

    fn children(&self) -> usize {
        1 + self.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.inner
        } else {
            self.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.inner
        } else {
            self.inner.get_mut_child(idx - 1)
        }
    }

    fn handle_input(&mut self, ctxt: &mut InputCtxt, event: InputEvent) -> bool {
        self.inner.handle_input(ctxt, event)
    }

    fn update(&mut self) {}
}
