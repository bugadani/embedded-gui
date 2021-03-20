use crate::{
    input::InputEvent,
    widgets::{ParentHolder, Widget, WidgetStateHolder},
    BoundingBox, InputCtxt, MeasureConstraint, MeasureSpec, MeasuredSize, Position,
};

pub trait HorizontalAlignment {
    fn horizontal_offset(width: u32, space: u32) -> i32;
}
pub trait VerticalAlignment {
    fn vertical_offset(height: u32, space: u32) -> i32;
}

pub struct Top;
pub struct Left;
pub struct Bottom;
pub struct Right;
pub struct Center;

impl HorizontalAlignment for Left {
    fn horizontal_offset(_width: u32, _space: u32) -> i32 {
        0
    }
}
impl HorizontalAlignment for Right {
    fn horizontal_offset(width: u32, space: u32) -> i32 {
        width as i32 - space as i32
    }
}
impl HorizontalAlignment for Center {
    fn horizontal_offset(width: u32, space: u32) -> i32 {
        (width as i32 - space as i32) / 2
    }
}

impl VerticalAlignment for Top {
    fn vertical_offset(_height: u32, _space: u32) -> i32 {
        0
    }
}
impl VerticalAlignment for Bottom {
    fn vertical_offset(height: u32, space: u32) -> i32 {
        height as i32 - space as i32
    }
}
impl VerticalAlignment for Center {
    fn vertical_offset(height: u32, space: u32) -> i32 {
        (height as i32 - space as i32) / 2
    }
}

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

pub struct FillParent<W, FD, H, V>
where
    FD: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    pub parent_index: Option<usize>,
    pub inner: W,
    pub direction: FD,
    pub bounds: BoundingBox,
    pub horizontal_alignment: H,
    pub vertical_alignment: V,
}

impl<W> FillParent<W, Horizontal, Center, Center>
where
    W: Widget,
{
    pub fn horizontal(inner: W) -> FillParent<W, Horizontal, Center, Top> {
        FillParent {
            parent_index: None,
            inner,
            direction: Horizontal,
            bounds: BoundingBox::default(),
            horizontal_alignment: Center,
            vertical_alignment: Top,
        }
    }

    pub fn vertical(inner: W) -> FillParent<W, Vertical, Left, Center> {
        FillParent {
            parent_index: None,
            inner,
            direction: Vertical,
            bounds: BoundingBox::default(),
            horizontal_alignment: Left,
            vertical_alignment: Center,
        }
    }

    pub fn both(inner: W) -> FillParent<W, HorizontalAndVertical, Center, Center> {
        FillParent {
            parent_index: None,
            inner,
            direction: HorizontalAndVertical,
            bounds: BoundingBox::default(),
            horizontal_alignment: Center,
            vertical_alignment: Center,
        }
    }
}

impl<W, D, H, V> FillParent<W, D, H, V>
where
    W: Widget,
    D: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    pub fn align_horizontal<H2>(self, align: H2) -> FillParent<W, D, H2, V>
    where
        H2: HorizontalAlignment,
    {
        FillParent {
            parent_index: self.parent_index,
            inner: self.inner,
            direction: self.direction,
            bounds: self.bounds,
            horizontal_alignment: align,
            vertical_alignment: self.vertical_alignment,
        }
    }
    pub fn align_vertical<V2>(self, align: V2) -> FillParent<W, D, H, V2>
    where
        V2: VerticalAlignment,
    {
        FillParent {
            parent_index: self.parent_index,
            inner: self.inner,
            direction: self.direction,
            bounds: self.bounds,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: align,
        }
    }
}

impl<W, D, H, V> WidgetStateHolder for FillParent<W, D, H, V>
where
    W: Widget,
    D: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn change_state(&mut self, state: u32) {
        // propagate state to child widget
        self.inner.change_state(state);
    }

    fn change_selection(&mut self, _state: bool) {}

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<W, D, H, V> Widget for FillParent<W, D, H, V>
where
    W: Widget,
    D: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn attach(&mut self, parent: Option<usize>, self_index: usize) {
        self.set_parent(parent);
        self.inner.attach(Some(self_index), self_index + 1);
    }

    fn arrange(&mut self, position: Position) {
        self.bounds.position = position;

        let inner_size = self.inner.bounding_box().size;

        self.inner.arrange(Position {
            x: position.x + H::horizontal_offset(self.bounds.size.width, inner_size.width),
            y: position.y + V::vertical_offset(self.bounds.size.height, inner_size.height),
        });
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

impl<W, D, H, V> ParentHolder for FillParent<W, D, H, V>
where
    W: Widget,
    D: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    fn set_parent(&mut self, index: Option<usize>) {
        self.parent_index = index;
    }
}
