use std::{thread, time::Duration};

use backend_embedded_graphics::{
    themes::{default::DefaultTheme, Theme},
    widgets::label::ascii::LabelConstructor,
    EgCanvas,
};
use embedded_graphics::{
    draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::Size as EgSize,
};
use embedded_graphics_simulator::{
    sdl2::MouseButton, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window as SimWindow,
};
use embedded_gui::{
    data::BoundData,
    geometry::Position,
    input::event::{InputEvent, PointerEvent, ScrollEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, row::Row},
        primitives::{border::Border, fill::FillParent},
        scroll::Scroll,
        slider::ScrollbarConnector,
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
    let display = SimulatorDisplay::new(EgSize::new(128, 96));

    let scroll_data = BoundData::new(ScrollbarConnector::new(), |_| ());
    let horizontal_scroll_data = BoundData::new(ScrollbarConnector::new(), |_| ());

    let mut gui = Window::new(
        EgCanvas::new(display),
        Column::new()
        .add(FillParent::horizontal(
            Label::new("Scroll down")
                .bind(&scroll_data)
                .on_data_changed(|label, data| {
                    label.text = if data.offset == data.maximum_offset {
                        "Scroll back"
                    } else if data.offset == 0 {
                        "Scroll down"
                    } else {
                        "Scroll more"
                    };
                }),
        ))
        .add(
            Column::new()
            .add(
                Scroll::horizontal(Label::new(
                    "Some very long text that can be used to demonstrate horizontal scrollbars",
                ))
                .set_active(false)
                .bind(&horizontal_scroll_data)
                .on_scroll_changed(ScrollbarConnector::on_scroll_widget_scroll_changed)
                .on_data_changed(ScrollbarConnector::on_scroll_widget_data_changed),
            )
            .add(
                DefaultTheme::horizontal_scrollbar()
                    .bind(&horizontal_scroll_data)
                    .on_data_changed(ScrollbarConnector::on_scrollbar_data_changed)
                    .on_value_changed(ScrollbarConnector::on_scrollbar_value_changed),
            ),
        )
        .add(
            Row::new()
                .add(Border::new(
                    Scroll::vertical(
                        Column::new()
                            .add(Label::new("S"))
                            .add(Label::new("c"))
                            .add(Label::new("r"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("l"))
                            .add(Label::new("o"))
                            .add(Label::new("Scrollolo :)"))
                            .add(
                                DefaultTheme::primary_button("Back to top")
                                    .bind(&scroll_data)
                                    .on_clicked(|data| data.scroll_to(0)),
                            ),
                    )
                    .friction(1)
                    .friction_divisor(2)
                    .bind(&scroll_data)
                    .on_scroll_changed(ScrollbarConnector::on_scroll_widget_scroll_changed)
                    .on_data_changed(ScrollbarConnector::on_scroll_widget_data_changed),
                ))
                .weight(8)
                .add(
                    DefaultTheme::vertical_scrollbar()
                        .bind(&scroll_data)
                        .on_data_changed(ScrollbarConnector::on_scrollbar_data_changed)
                        .on_value_changed(ScrollbarConnector::on_scrollbar_value_changed),
                ),
        ),
    );

    fn print_type_of<T>(_: &T) {
        println!("Type of tree: {}", std::any::type_name::<T>());
        println!("Length of type: {}", std::any::type_name::<T>().len());
        println!("Size of struct: {}", std::mem::size_of::<T>());
    }

    print_type_of(&gui.root);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = SimWindow::new("GUI demonstration", &output_settings);

    // In this example, the size of the widgets can't change so it's enough to measure once.
    gui.measure();

    loop {
        gui.canvas
            .target
            .clear(BinaryColor::BACKGROUND_COLOR)
            .unwrap();

        gui.update();
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
