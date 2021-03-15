#![no_std]

use core::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct MeasuredSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub position: Position,
    pub size: MeasuredSize,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            size: MeasuredSize {
                width: 0,
                height: 0,
            },
        }
    }
}

pub trait WidgetRenderer<C: Canvas>: Widget {
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error>;
}

pub struct NoRenderer;

pub enum NoCanvas {}
impl Canvas for NoCanvas {
    type Error = ();
}

pub trait Canvas {
    type Error;
}

pub enum MeasureConstraint {
    AtMost(u32),
    Exactly(u32),
    Unspecified,
}

pub struct MeasureSpec {
    width: MeasureConstraint,
    height: MeasureConstraint,
}

pub trait Widget {
    //type InputController: InputController;

    //fn input_event(&mut self, event: <Self::InputController as InputController>::Event) -> bool;
    fn widget_properties(&mut self) -> &mut WidgetProperties;

    fn bounding_box(&self) -> BoundingBox;

    fn bounding_box_mut(&mut self) -> &mut BoundingBox;

    fn width(mut self, width: Size) -> Self
    where
        Self: Sized,
    {
        self.widget_properties().width = width;
        self
    }

    fn height(mut self, height: Size) -> Self
    where
        Self: Sized,
    {
        self.widget_properties().height = height;
        self
    }

    fn measure(&mut self, measure_spec: MeasureSpec);

    fn arrange(&mut self, position: Position) {
        self.bounding_box_mut().position = position;
    }

    fn set_measured_size(&mut self, size: MeasuredSize) {
        self.bounding_box_mut().size = size;
    }
}

pub enum Size {
    WrapContent,
    FillParent,
    Exact(u32),
}

pub struct WidgetProperties {
    width: Size,
    height: Size,
}

impl Default for WidgetProperties {
    fn default() -> Self {
        Self {
            width: Size::WrapContent,
            height: Size::WrapContent,
        }
    }
}

pub trait LabelProperties<C: Canvas> {
    fn measure_text(&self, text: &str) -> MeasuredSize;
}

pub trait LabelConstructor<C, P> {
    fn new(text: &'static str) -> Label<C, P>
    where
        C: Canvas,
        P: LabelProperties<C>;
}

pub struct Label<C, P>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    // FIXME: use heapless::String
    pub text: &'static str,
    pub widget_properties: WidgetProperties,
    pub label_properties: P,
    pub bounds: BoundingBox,
    pub _marker: PhantomData<C>,
}

impl<C, P> Widget for Label<C, P>
where
    C: Canvas,
    P: LabelProperties<C>,
{
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let size = self.label_properties.measure_text(self.text);

        let width = match measure_spec.width {
            MeasureConstraint::AtMost(width) => width.min(size.width),
            MeasureConstraint::Exactly(width) => width,
            Unspecified => size.width,
        };

        let height = match measure_spec.height {
            MeasureConstraint::AtMost(height) => height.min(size.height),
            MeasureConstraint::Exactly(height) => height,
            Unspecified => size.height,
        };

        self.set_measured_size(MeasuredSize { width, height })
    }
}

pub struct Button<I: Widget> {
    pub widget_properties: WidgetProperties,
    pub inner: I,
}

impl<I> Button<I>
where
    I: Widget,
{
    pub fn new(inner: I) -> Self {
        Self {
            widget_properties: WidgetProperties::default(),
            inner,
        }
    }
}

impl<I: Widget> Widget for Button<I> {
    fn widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn bounding_box(&self) -> BoundingBox {
        self.inner.bounding_box()
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        self.inner.bounding_box_mut()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec)
    }
}

pub struct Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub canvas: C,
    pub root: W,
}

impl<C, W> Window<C, W>
where
    C: Canvas,
    W: Widget + WidgetRenderer<C>,
{
    pub fn new(canvas: C, root: W) -> Self {
        Self { canvas, root }
    }

    pub fn measure(&mut self) {
        self.root.measure(MeasureSpec {
            width: MeasureConstraint::AtMost(0),
            height: MeasureConstraint::AtMost(0),
        });
    }

    pub fn arrange(&mut self) {
        self.root.arrange(Position { x: 0, y: 0 });
    }

    pub fn draw(&mut self) -> Result<(), C::Error> {
        self.root.draw(&mut self.canvas)
    }
}
