use crate::{
    geometry::{
        axis_order::{AxisOrder, Horizontal as HorizontalHelper, Vertical as VerticalHelper},
        measurement::MeasureSpec,
        BoundingBox, MeasuredSize, Position,
    },
    input::event::InputEvent,
    state::WidgetState,
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetStateHolder},
    Canvas, WidgetRenderer,
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
    type AxisOrder: AxisOrder;

    fn measure<W: Widget>(widget: &mut W, child_size: MeasuredSize, measure_spec: MeasureSpec) {
        let main_child_size =
            <Self::AxisOrder as AxisOrder>::main_axis(child_size.width, child_size.height);
        let cross_child_size =
            <Self::AxisOrder as AxisOrder>::cross_axis(child_size.width, child_size.height);
        let main_spec =
            <Self::AxisOrder as AxisOrder>::main_axis(measure_spec.width, measure_spec.height);

        let main = main_spec.largest().unwrap_or(main_child_size);
        let (width, height) = <Self::AxisOrder as AxisOrder>::merge(main, cross_child_size);

        widget.set_measured_size(MeasuredSize { width, height })
    }
}

pub struct Horizontal;
pub struct Vertical;
pub struct HorizontalAndVertical;

impl FillDirection for Horizontal {
    type AxisOrder = HorizontalHelper;
}

impl FillDirection for Vertical {
    type AxisOrder = VerticalHelper;
}

impl FillDirection for HorizontalAndVertical {
    type AxisOrder = HorizontalHelper; // This isn't true but it's not used

    fn measure<W: Widget>(widget: &mut W, child_size: MeasuredSize, measure_spec: MeasureSpec) {
        let width = measure_spec.width.largest().unwrap_or(child_size.width);
        let height = measure_spec.height.largest().unwrap_or(child_size.height);

        widget.set_measured_size(MeasuredSize { width, height })
    }
}

pub struct FillParent<W, FD, H, V>
where
    FD: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
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
            inner,
            direction: Horizontal,
            bounds: BoundingBox::default(),
            horizontal_alignment: Center,
            vertical_alignment: Top,
        }
    }

    pub fn vertical(inner: W) -> FillParent<W, Vertical, Left, Center> {
        FillParent {
            inner,
            direction: Vertical,
            bounds: BoundingBox::default(),
            horizontal_alignment: Left,
            vertical_alignment: Center,
        }
    }

    pub fn both(inner: W) -> FillParent<W, HorizontalAndVertical, Center, Center> {
        FillParent {
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
    fn on_state_changed(&mut self, state: WidgetState) {
        self.inner.on_state_changed(state);
    }

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
    fn attach(&mut self, parent: usize, self_index: usize) {
        self.inner.attach(parent, self_index);
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
        self.inner.measure(MeasureSpec {
            width: measure_spec.width.to_at_most(),
            height: measure_spec.height.to_at_most(),
        });

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

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We just relay whatever the child desires
        self.inner.test_input(event).map(|i| i + 1)
    }
}

impl<W, D, H, V> ParentHolder for FillParent<W, D, H, V>
where
    W: Widget,
    D: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn parent_index(&self) -> usize {
        self.inner.parent_index()
    }
}

impl<W, D, H, V> UpdateHandler for FillParent<W, D, H, V>
where
    W: Widget,
    D: FillDirection,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn update(&mut self) {
        self.inner.update();
    }
}

impl<C, W, FD, H, V> WidgetRenderer<C> for FillParent<W, FD, H, V>
where
    FD: FillDirection,
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
    H: HorizontalAlignment,
    V: VerticalAlignment,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.inner.draw(canvas)
    }
}
