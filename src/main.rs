use std::path::Path;

use relm4::RelmApp;
use relm4::gtk;

fn main() {
    let app = RelmApp::new("com.github.tiago-vargas.rust-chip-8-interpreter");
    app.run::<App>(());
}

use gtk::cairo::{Context, Operator};
use gtk::prelude::*;
use relm4::drawing::DrawHandler;
use relm4::{Component, ComponentParts, ComponentSender, RelmWidgetExt};
use rust_chip_8_interpreter::interpreter::Machine;

#[derive(Debug)]
enum Message {
    Draw,
}

#[derive(Debug)]
struct UpdatePointsMessage;

struct App {
    width: f64,
    height: f64,
    handler: DrawHandler,
    buffer: [[u8; 64]; 32],
    machine: Machine,
}

#[relm4::component]
impl Component for App {
    type Init = ();
    type Input = Message;
    type Output = ();
    type CommandOutput = UpdatePointsMessage;

    view! {
        gtk::Window {
            set_default_size: (600, 600),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 12,
                set_hexpand: true,

                #[local_ref]
                area -> gtk::DrawingArea {
                    set_vexpand: true,
                    set_hexpand: true,

                    add_controller = gtk::GestureClick {
                        set_button: 0,
                        connect_pressed[sender] => move |controller, _, _x, _y| {
                            if controller.current_button() == gtk::gdk::BUTTON_PRIMARY {
                                // sender.input(Message::Draw(model.buffer));
                                sender.input(Message::Draw);
                            }
                        }
                    },
                    // connect_resize[sender] => move |_, x, y| {
                    //     sender.input(Message::Resize((x, y)));
                    // }
                },
            }
        }
    }

    fn update(&mut self, message: Message, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let cx = self.handler.get_context();

        match message {
            // Message::ClearScreen => {},
            Message::Draw => {
                let buffer = [
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],  //                        ████████████████████████   ███████████████████████████         ███████████████                           ███████████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                        ████████████████████████   █████████████████████████████████   ██████████████████                     ██████████████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                              ████████████               █████████         █████████         ███████████████               ███████████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                              ████████████               ███████████████████████████         █████████████████████   █████████████████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                              ████████████               ███████████████████████████         █████████   █████████████████████   █████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                              ████████████               █████████         █████████         █████████      ███████████████      █████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                        ████████████████████████   █████████████████████████████████   ██████████████████      █████████      ██████████████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],  //                        ████████████████████████   ███████████████████████████         ██████████████████         ███         ██████████████████
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                    [0; 64],
                ];

                draw(&cx, &buffer);
            },
            // Message::Draw(buffer) => {
            //     // self.buffer = self.machine.video_buffer;
            //     draw(&cx, &buffer);
            // },

            // Message::AddPoint((x, y)) => {
            //     self.points.push(Point::new(x, y));
            // }
            // Message::Resize((x, y)) => {
            //     self.width = x as f64;
            //     self.height = y as f64;
            // }
            // Message::Reset => {
            //     cx.set_operator(Operator::Clear);
            //     cx.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            //     cx.paint().expect("Couldn't fill context");
            // }
        }

        // draw(&cx, &self.points);
    }

    fn update_cmd(&mut self, _: UpdatePointsMessage, _: ComponentSender<Self>, _root: &Self::Root) {
        // for point in &mut self.points {
        //     let Point { x, y, .. } = point;
        //     if *x < 0.0 {
        //         point.xs = point.xs.abs();
        //     } else if *x > self.width {
        //         point.xs = -point.xs.abs();
        //     }
        //     *x = x.clamp(0.0, self.width);
        //     *x += point.xs;

        //     if *y < 0.0 {
        //         point.ys = point.ys.abs();
        //     } else if *y > self.height {
        //         point.ys = -point.ys.abs();
        //     }
        //     *y = y.clamp(0.0, self.height);
        //     *y += point.ys;
        // }

        let cx = self.handler.get_context();
        // draw(&cx, &self.points);
        // draw(&cx, &self.buffer);
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            width: 100.0,
            height: 100.0,
            handler: DrawHandler::new(),
            buffer: [[0; 64]; 32],
            machine: Machine::new(),
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();

        // model.machine.load_rom(Path::new("./assets/IBM Logo.ch8"));

        ComponentParts { model, widgets }
    }
}

// fn draw(cx: &Context, points: &[Point]) {
//     for point in points {
//         let Point {
//             x,
//             y,
//             color: Color { r, g, b },
//             ..
//         } = *point;
//         cx.set_source_rgb(r, g, b);
//         cx.arc(x, y, 10.0, 0.0, std::f64::consts::PI * 2.0);
//         cx.fill().expect("Couldn't fill arc");
//     }
// }

fn draw(cx: &Context, buffer: &[[u8; 64]; 32]) {
    cx.set_source_rgb(0.0, 0.0, 0.0);  // Black
    cx.paint();  // Make the canvas black

    // Canvas: 512 px x 512 px
    let cell_width = 8.0;
    let cell_height = 16.0;

    cx.set_source_rgb(1.0, 1.0, 1.0);  // White
    for (line, row) in buffer.iter().enumerate() {
        for (column, byte) in row.iter().enumerate() {
            if *byte == 1 {
                let x = column as f64;
                let y = line as f64;
                cx.rectangle(x * cell_width, y * cell_height, cell_width, cell_height);
                cx.fill();
            }
        }
    }
}
