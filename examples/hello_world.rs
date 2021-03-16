use backend_embedded_graphics::EgCanvas;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size as EgSize};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window as SimWindow,
};
use embedded_gui::{
    data::WidgetData,
    widgets::{
        button::Button,
        label::{Label, LabelConstructor},
        DataHolder, Widget,
    },
    Size, Window,
};

fn main() {
    let display = SimulatorDisplay::<BinaryColor>::new(EgSize::new(256, 256));

    let counter = WidgetData::new(true, |_| {});
    let mut gui = Window::new(
        EgCanvas::new(display),
        Button::new(
            Label::new("foobar")
                .bind(&counter)
                .on_data_changed(|mut widget, data| {
                    widget.text = if *data.get() { "on" } else { "off" };
                }),
        )
        .bind(&counter)
        .on_clicked(|_, data| {
            data.update(|mut data| {
                *data = !*data;
                true
            });
        })
        .width(Size::FillParent),
    );

    gui.measure();
    gui.arrange();
    gui.draw().unwrap();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    SimWindow::new("Hello GUI", &output_settings).show_static(&gui.canvas.target);
}
