use std::{thread, time::Duration};

use backend_embedded_graphics::EgCanvas;
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window as SimWindow,
};
use embedded_gui::{
    data::{BoundData, WidgetData},
    input::InputEvent,
    widgets::{
        button::Button,
        label::{Label, LabelConstructor},
        DataHolder, Widget,
    },
    Position, Size, Window,
};

fn convert_input(event: SimulatorEvent) -> Result<InputEvent, bool> {
    match event {
        SimulatorEvent::MouseButtonUp {
            mouse_btn: MouseButton::Left,
            point,
        } => Ok(InputEvent::PointerUp(Position {
            x: point.x,
            y: point.y,
        })),
        SimulatorEvent::MouseButtonDown {
            mouse_btn: MouseButton::Left,
            point,
        } => Ok(InputEvent::PointerDown(Position {
            x: point.x,
            y: point.y,
        })),
        SimulatorEvent::MouseMove { point } => Ok(InputEvent::PointerHover(Position {
            x: point.x,
            y: point.y,
        })),
        SimulatorEvent::Quit => Err(true),
        _ => Err(false),
    }
}

fn main() {
    let display = SimulatorDisplay::<BinaryColor>::new(EgSize::new(256, 256));

    let counter = BoundData::new(true, |data| {
        println!("Data changed to {:?}", data);
    });

    let mut gui = Window::new(
        EgCanvas::new(display),
        Button::new(
            Label::new("foobar")
                .bind(&counter)
                .on_data_changed(|mut widget, data| {
                    // FIXME: not called
                    widget.text = if *data.get() { "on" } else { "off" };
                }),
        )
        .bind(&counter)
        .on_clicked(|data| {
            data.update(|mut data| {
                *data = !*data;
                true
            });
            println!("Clicked!");
        })
        .width(Size::FillParent),
    );

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = SimWindow::new("TextBox demonstration", &output_settings);

    loop {
        gui.canvas.target.clear(BinaryColor::Off).unwrap();

        gui.update();
        gui.measure();
        gui.arrange();
        gui.draw().unwrap();

        // Update the window.
        window.update(&gui.canvas.target);

        // Handle key and mouse events.
        for event in window.events() {
            match convert_input(event) {
                Ok(input) => {
                    gui.input_event(input);
                }
                Err(true) => return,
                _ => {}
            }
        }

        // Wait for a little while.
        thread::sleep(Duration::from_millis(10));
    }
}
