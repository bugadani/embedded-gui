use embedded_canvas::{ CCanvasAt};
use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point, Dimensions, Size},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    prelude::WrapperBindable,
    state::WidgetState,
    widgets::Widget,
    WidgetRenderer,
};

use crate::{themes::Theme, EgCanvas};

pub trait CanvasProperties {
    type Color;
    type Canvas: Drawable<Color = Self::Color>;

    fn canvas(&mut self) -> &mut Self::Canvas;
    fn measure(&self) -> MeasuredSize;
    fn move_canvas(&mut self, pos: Position);
}

pub struct CCanvasStyle<C, const W: usize, const H: usize>
where
    C: PixelColor,
{
    pub clear_color: C,
    pub canvas: CCanvasAt<C, W, H>,
}

impl<C, const W: usize, const H: usize> CanvasProperties for CCanvasStyle<C, W, H>
where
    C: PixelColor,
{
    type Color = C;
    type Canvas = CCanvasAt<C, W, H>;

    fn canvas(&mut self) -> &mut Self::Canvas {
        &mut self.canvas
    }

    fn measure(&self) -> MeasuredSize {
        MeasuredSize {
            width: W as u32,
            height: H as u32,
        }
    }

    fn move_canvas(&mut self, pos: Position) {
        self.canvas.top_left = Point { x: pos.x, y: pos.y };
    }
}

impl<C, const W: usize, const H: usize> Default for CCanvasStyle<C, W, H>
where
    C: Theme,
{
    fn default() -> Self {
        Self {
            clear_color: C::BACKGROUND_COLOR,
            canvas: CCanvasAt::new(Point::zero()),
        }
    }
}

//#[cfg(feature = "std")]
use embedded_canvas::CanvasAt;

//#[cfg(feature = "std")]
pub struct CanvasStyle<C>
where
C: PixelColor,
{
    pub clear_color: C,
    pub canvas: CanvasAt<C>,
}


//#[cfg(feature = "std")]
impl<C> CanvasStyle<C>
where
    C: Theme,
{
    pub fn new(size: Size) -> Self {
        Self {
            clear_color: C::BACKGROUND_COLOR,
            canvas: CanvasAt::new(Point::zero(), size),
        }
    }
}

//#[cfg(feature = "std")]
impl<C> CanvasProperties for CanvasStyle<C>
where
    C: PixelColor,
{
    type Color = C;
    type Canvas = CanvasAt<C>;

    fn canvas(&mut self) -> &mut Self::Canvas {
        &mut self.canvas
    }

    fn measure(&self) -> MeasuredSize {
        MeasuredSize {
            width: self.canvas.bounding_box().size.width,
            height: self.canvas.bounding_box().size.height,
        }
    }

    fn move_canvas(&mut self, pos: Position) {
        self.canvas.top_left = Point { x: pos.x, y: pos.y };
    }
}

//#[cfg(feature = "std")]
impl<C> Default for CanvasStyle<C>
where
    C: Theme,
{
    fn default() -> Self {
        Self {
            clear_color: C::BACKGROUND_COLOR,
            canvas: CanvasAt::new(Point::zero(), Size::zero()),
        }
    }
}

pub struct Canvas<P> {
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub canvas_properties: P,
}

impl<P> Canvas<P> {
    pub fn new() -> Self
    where
        P: Default,
    {
        Self {
            parent_index: 0,
            bounds: BoundingBox::default(),
            canvas_properties: P::default(),
        }
    }

    pub fn with_properties(properties: P) -> Self
    where
        P: Default,
    {
        Self {
            parent_index: 0,
            bounds: BoundingBox::default(),
            canvas_properties: properties,
        }
    }

    pub fn canvas(&mut self) -> &mut P::Canvas
    where
        P: CanvasProperties,
    {
        self.canvas_properties.canvas()
    }
}

impl<P> WrapperBindable for Canvas<P> where P: CanvasProperties {}

impl<P> Widget for Canvas<P>
where
    P: CanvasProperties,
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let canvas_size = self.canvas_properties.measure();

        let width = measure_spec.width.apply_to_measured(canvas_size.width);
        let height = measure_spec.height.apply_to_measured(canvas_size.height);

        self.bounds.size = MeasuredSize { width, height };
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }

    fn on_state_changed(&mut self, _: WidgetState) {}
}

impl<C, DT, P> WidgetRenderer<EgCanvas<DT>> for Canvas<P>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    P: CanvasProperties<Color = C>,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.canvas_properties.move_canvas(self.bounds.position);
        self.canvas().draw(&mut canvas.target)?;

        Ok(())
    }
}
