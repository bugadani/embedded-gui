use embedded_graphics::pixelcolor::PixelColor;

// TODO: rename DefaultTheme to LightTheme and add DarkTheme
pub mod default;
pub mod light;

// TODO: merge this into DefaultTheme. This would allow defining different color schemes for the same
// color space.
pub trait Theme: PixelColor {
    const TEXT_COLOR: Self;
    const BORDER_COLOR: Self;
    const BACKGROUND_COLOR: Self;
}
