use std::{fmt::Write, thread, time::Duration};

use backend_embedded_graphics::{
    themes::{default::DefaultTheme, Theme},
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
    data::{BoundData, WidgetData},
    geometry::Position,
    input::event::{InputEvent, PointerEvent, ScrollEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, row::Row},
        primitives::{fill::FillParent, spacing::Spacing},
    },
    Window,
};
use heapless::String;

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
            SimulatorEvent::MouseWheel { scroll_delta, .. } => {
                // TODO: We could break this down into multiple scroll events
                Ok(InputEvent::ScrollEvent(if scroll_delta.y != 0 {
                    ScrollEvent::VerticalScroll(scroll_delta.y * 4)
                } else {
                    ScrollEvent::HorizontalScroll(scroll_delta.x * 4)
                }))
            }
            SimulatorEvent::Quit => Err(true),
            _ => Err(false),
        }
    }
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(192, 160));

    let radio = BoundData::new(0, |_| ());
    let checkbox = BoundData::new(false, |_| ());
    let slider1_data = BoundData::new(0, |_| ());
    let slider2_data = BoundData::new(0, |_| ());
    let everything = BoundData::new((&radio, &slider1_data, &slider2_data, &checkbox), |_| ());

    let mut gui = Window::new(
        EgCanvas::new(display),
        Spacing::new(
            Column::new()
                .spacing(1)
                .add(Label::new("Checkboxes and radio buttons").text_color(Rgb888::BLACK))
                .add(
                    DefaultTheme::check_box("Check me")
                        .bind(&checkbox)
                        .on_selected_changed(|checked, data| *data = checked),
                )
                .add(
                    DefaultTheme::check_box("Inactive")
                        .bind(&checkbox)
                        .active(false)
                        .on_data_changed(|checkbox, data| checkbox.set_checked(*data)),
                )
                .add(
                    DefaultTheme::radio_button("Can't select me")
                        .bind(&radio)
                        .on_selected_changed(|_, data| *data = 0)
                        .on_data_changed(|radio, data| radio.set_checked(*data == 0))
                        .active(false),
                )
                .add(
                    DefaultTheme::radio_button("Select me")
                        .bind(&radio)
                        .on_selected_changed(|_, data| *data = 0)
                        .on_data_changed(|radio, data| radio.set_checked(*data == 0)),
                )
                .add(
                    DefaultTheme::radio_button("... or me!")
                        .bind(&radio)
                        .on_selected_changed(|_, data| *data = 1)
                        .on_data_changed(|radio, data| radio.set_checked(*data == 1)),
                )
                .add(Spacing::new(Label::new("Numeric sliders")).top(4))
                .add(
                    Row::new()
                        .add(FillParent::horizontal(
                            Label::new(String::<11>::from("0"))
                                .bind(&slider1_data)
                                .on_data_changed(|label, data| {
                                    label.text.clear();
                                    write!(label.text, "{}", data).unwrap();
                                }),
                        ))
                        .weight(1)
                        .add(
                            Spacing::new(
                                DefaultTheme::slider(-100..=100)
                                    .bind(&slider1_data)
                                    .on_value_changed(|data, value| *data = value)
                                    .on_data_changed(|slider, data| slider.set_value(*data)),
                            )
                            .top(1),
                        )
                        .weight(4),
                )
                .add(
                    Row::new()
                        .add(FillParent::horizontal(
                            Label::new(String::<11>::from("0"))
                                .bind(&slider2_data)
                                .on_data_changed(|label, data| {
                                    label.text.clear();
                                    write!(label.text, "{}", data).unwrap();
                                }),
                        ))
                        .weight(1)
                        .add(
                            Spacing::new(
                                DefaultTheme::slider(0..=5)
                                    .bind(&slider2_data)
                                    .on_value_changed(|data, value| *data = value)
                                    .on_data_changed(|slider, data| slider.set_value(*data)),
                            )
                            .top(1),
                        )
                        .weight(4),
                )
                .add(
                    Row::new().add(Label::new("Inactive")).add(
                        Spacing::new(
                            DefaultTheme::slider(0..=5)
                                .set_active(false)
                                .bind(&slider2_data)
                                .on_value_changed(|data, value| *data = value)
                                .on_data_changed(|slider, data| slider.set_value(*data)),
                        )
                        .top(1),
                    ),
                )
                .add(
                    DefaultTheme::primary_button("Reset")
                        .bind(&everything)
                        .on_clicked(|data| {
                            data.0.update(|data| *data = 0);
                            data.1.update(|data| *data = 0);
                            data.2.update(|data| *data = 0);
                            data.3.update(|data| *data = false);
                        }),
                ),
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
    let mut window = SimWindow::new("Everything but the kitchen sink", &output_settings);

    loop {
        gui.canvas.target.clear(Rgb888::BACKGROUND_COLOR).unwrap();

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
