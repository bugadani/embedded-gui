use std::{thread, time::Duration};

use backend_embedded_graphics::{
    themes::default::DefaultTheme,
    widgets::label::{ascii::LabelConstructor, LabelStyling},
    EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb888, RgbColor},
    prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window as SimWindow,
};
use embedded_gui::{
    data::BoundData,
    geometry::Position,
    input::event::{InputEvent, PointerEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, Cell},
        primitives::spacing::Spacing,
    },
    Window,
};

fn convert_input(event: SimulatorEvent) -> Result<InputEvent, bool> {
    unsafe {
        // This is fine for a demo
        static mut MOUSE_DOWN: bool = false;
        match event {
            SimulatorEvent::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                point,
            } => {
                MOUSE_DOWN = false;
                Ok(InputEvent::PointerEvent(
                    Position {
                        x: point.x,
                        y: point.y,
                    },
                    PointerEvent::Up,
                ))
            }
            SimulatorEvent::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                point,
            } => {
                MOUSE_DOWN = true;
                Ok(InputEvent::PointerEvent(
                    Position {
                        x: point.x,
                        y: point.y,
                    },
                    PointerEvent::Down,
                ))
            }
            SimulatorEvent::MouseMove { point } => Ok(InputEvent::PointerEvent(
                Position {
                    x: point.x,
                    y: point.y,
                },
                if MOUSE_DOWN {
                    PointerEvent::Drag
                } else {
                    PointerEvent::Hover
                },
            )),
            SimulatorEvent::Quit => Err(true),
            _ => Err(false),
        }
    }
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(192, 160));

    let radio = BoundData::new(0, |_| ());

    let mut gui = Window::new(
        EgCanvas::new(display),
        Spacing::new(
            Column::new(Cell::new(
                Label::new("Checkboxes and radio buttons").text_color(Rgb888::BLACK),
            ))
            .spacing(1)
            .add(Cell::new(DefaultTheme::check_box("Inactive").active(false)))
            .add(Cell::new(
                DefaultTheme::check_box("Inactive, checked")
                    .checked(true)
                    .active(false),
            ))
            .add(Cell::new(DefaultTheme::check_box("Check me")))
            .add(Cell::new(
                DefaultTheme::radio_button("Can't select me")
                    .bind(&radio)
                    .on_selected_changed(|_, data| *data = 0)
                    .on_data_changed(|radio, data| {
                        radio.set_checked(*data == 0);
                    })
                    .active(false),
            ))
            .add(Cell::new(
                DefaultTheme::radio_button("Select me")
                    .bind(&radio)
                    .on_selected_changed(|_, data| *data = 0)
                    .on_data_changed(|radio, data| {
                        radio.set_checked(*data == 0);
                    }),
            ))
            .add(Cell::new(
                DefaultTheme::radio_button("... or me!")
                    .bind(&radio)
                    .on_selected_changed(|_, data| *data = 1)
                    .on_data_changed(|radio, data| {
                        radio.set_checked(*data == 1);
                    }),
            )),
        )
        .all(2),
    );

    println!("Size of struct: {}", std::mem::size_of_val(&gui.root));
    fn print_type_of<T>(_: &T) {
        println!("type of tree: {}", std::any::type_name::<T>());
        println!("length of type: {}", std::any::type_name::<T>().len());
    }

    print_type_of(&gui.root);

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = SimWindow::new("Checkboxes and radio buttons", &output_settings);

    loop {
        gui.canvas.target.clear(Rgb888::WHITE).unwrap();

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
