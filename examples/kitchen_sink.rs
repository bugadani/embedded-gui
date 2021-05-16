use std::{fmt::Write, thread, time::Duration};

use backend_embedded_graphics::{
    themes::{default::DefaultTheme, Theme},
    widgets::textbox::{ascii::TextBoxConstructor, TextBoxStyling},
    widgets::{label::ascii::LabelConstructor, textbox::CenterAligned},
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
    data::{BoundData, WidgetData},
    geometry::Position,
    input::event::{InputEvent, PointerEvent, ScrollEvent},
    widgets::{
        label::Label,
        layouts::linear::{column::Column, row::Row},
        primitives::{border::Border, fill::FillParent, spacing::Spacing, visibility::Visibility},
        scroll::Scroll,
        slider::ScrollbarConnector,
        textbox::TextBox,
        wrapper::WrapperBindable,
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

#[derive(PartialEq)]
enum Page {
    Textbox,
    Check,
    Slider,
    Scroll,
}

fn main() {
    let display = SimulatorDisplay::new(EgSize::new(240, 180));

    let page = BoundData::new(Page::Textbox, |_| ());

    let radio = BoundData::new(0, |_| ());
    let checkbox = BoundData::new(false, |_| ());
    let toggle = BoundData::new(false, |_| ());
    let checkables = BoundData::new((&radio, &checkbox, &toggle), |_| ());

    let slider1_data = BoundData::new(0, |_| ());
    let slider2_data = BoundData::new(0, |_| ());
    let sliders = BoundData::new((&slider1_data, &slider2_data), |_| ());

    let scroll_data = BoundData::new(ScrollbarConnector::new(), |_| ());
    let horizontal_scroll_data = BoundData::new(ScrollbarConnector::new(), |_| ());

    let tabs = Row::new()
        .spacing(1)
        .add(
            DefaultTheme::toggle_button("Textbox")
                .disallow_manual_uncheck()
                .bind(&page)
                .on_selected_changed(|_, page| *page = Page::Textbox)
                .on_data_changed(|toggle, data| toggle.set_checked(*data == Page::Textbox)),
        )
        .add(
            DefaultTheme::toggle_button("Checkables")
                .disallow_manual_uncheck()
                .bind(&page)
                .on_selected_changed(|_, page| *page = Page::Check)
                .on_data_changed(|toggle, data| toggle.set_checked(*data == Page::Check)),
        )
        .add(
            DefaultTheme::toggle_button("Sliders")
                .disallow_manual_uncheck()
                .bind(&page)
                .on_selected_changed(|_, page| *page = Page::Slider)
                .on_data_changed(|toggle, data| toggle.set_checked(*data == Page::Slider)),
        )
        .add(
            DefaultTheme::toggle_button("Scrolling")
                .disallow_manual_uncheck()
                .bind(&page)
                .on_selected_changed(|_, page| *page = Page::Scroll)
                .on_data_changed(|toggle, data| toggle.set_checked(*data == Page::Scroll)),
        );

    let textbox_page = Border::new(FillParent::both(
        TextBox::new(
            "Some \x1b[4mstylish\x1b[24m multiline text that expands the widget vertically",
        )
        .horizontal_alignment(CenterAligned)
        .vertical_alignment(CenterAligned),
    ));

    let checkables_page = Column::new()
        .spacing(1)
        .add(Label::new("Checkboxes and radio buttons"))
        .add(
            DefaultTheme::check_box("Check me")
                .bind(&checkbox)
                .on_selected_changed(|checked, data| *data = checked)
                .on_data_changed(|checkbox, data| checkbox.set_checked(*data)),
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
        .add(
            DefaultTheme::toggle_button("Click me!")
                .bind(&toggle)
                .on_selected_changed(|selected, data| *data = selected)
                .on_data_changed(|toggle, data| toggle.set_checked(*data)),
        )
        .add(
            Visibility::new(Label::new("Toggle checked"))
                .bind(&toggle)
                .on_data_changed(|widget, data| widget.set_visible(*data)),
        )
        .add(
            DefaultTheme::primary_button("Reset")
                .bind(&checkables)
                .on_clicked(|data| {
                    data.0.update(|data| *data = 0);
                    data.1.update(|data| *data = false);
                    data.2.update(|data| *data = false);
                }),
        );

    let sliders_page = Column::new()
        .spacing(1)
        .add(Label::new("Numeric sliders"))
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
                .bind(&sliders)
                .on_clicked(|data| {
                    data.0.update(|data| *data = 0);
                    data.1.update(|data| *data = 0);
                }),
        );

    let scrolling_page = Column::new()
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
            Row::new()
                .add(Border::new(
                    Scroll::vertical(
                        Spacing::new(
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
                        .all(2),
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
        )
        .weight(1)
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
        );

    let mut gui = Window::new(
        EgCanvas::new(display),
        Spacing::new(
            Column::new()
                .add(tabs)
                .add(
                    Visibility::new(textbox_page)
                        .bind(&page)
                        .on_data_changed(|widget, page| widget.set_visible(*page == Page::Textbox)),
                )
                .weight(1) // TODO replace with a Layer widget
                .add(
                    Visibility::new(checkables_page)
                        .bind(&page)
                        .on_data_changed(|widget, page| widget.set_visible(*page == Page::Check)),
                )
                .add(
                    Visibility::new(sliders_page)
                        .bind(&page)
                        .on_data_changed(|widget, page| widget.set_visible(*page == Page::Slider)),
                )
                .add(
                    Visibility::new(scrolling_page)
                        .bind(&page)
                        .on_data_changed(|widget, page| widget.set_visible(*page == Page::Scroll)),
                ),
        )
        .all(2),
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
    let mut window = SimWindow::new("Everything but the kitchen sink", &output_settings);

    loop {
        gui.canvas
            .target
            .clear(BinaryColor::BACKGROUND_COLOR)
            .unwrap();

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
