use embedded_graphics::pixelcolor::PixelColor;

pub mod default;

pub trait Theme: PixelColor {
    const TEXT_COLOR: Self;
    const BORDER_COLOR: Self;
    const BACKGROUND_COLOR: Self;
}
