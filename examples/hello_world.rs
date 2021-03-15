use backend_embedded_graphics::EgCanvas;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size as EgSize};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window as SimWindow,
};
use embedded_gui::{
    widgets::{
        button::Button,
        label::{Label, LabelConstructor},
        Widget,
    },
    Size, Window,
};

fn main() {
    let display = SimulatorDisplay::<BinaryColor>::new(EgSize::new(256, 256));

    let mut gui = Window::new(
        EgCanvas::new(display),
        Button::new(Label::new("foobar")).width(Size::FillParent),
    );

    gui.measure();
    gui.arrange();
    gui.draw().unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    SimWindow::new("Hello GUI", &output_settings).show_static(&gui.canvas.target);
}
