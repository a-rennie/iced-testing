use ascii_crypt::{decode, encode};
use iced::widget::{button, column, image, row, text, text_input};
use iced::{alignment, window, Alignment, Color, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (750, 750),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };

    Crypt::run(settings)
}

struct Crypt {
    value: String,
    error: String,
    crab: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Encode,
    Decode,
    InputChanged(String),
    Crab,
}

impl Sandbox for Crypt {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: String::new(),
            error: String::new(),
            crab: false,
        }
    }

    fn title(&self) -> String {
        String::from("Encoder/Decoder")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Encode => {
                self.value = match encode(&self.value) {
                    Ok(message) => message,
                    Err(err) => {
                        self.error = err.to_string();
                        String::from("")
                    }
                };
            }
            Message::Decode => {
                self.value = match decode(&self.value) {
                    Ok(message) => message,
                    Err(err) => {
                        self.error = err.to_string();
                        String::from("")
                    }
                };
            }
            Message::InputChanged(str) => self.value = str,
            Message::Crab => self.crab = !self.crab,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("Type something...", &self.value, Message::InputChanged),
            row![
                button(text("Decode").horizontal_alignment(alignment::Horizontal::Center))
                    .width(Length::Fill)
                    .on_press(Message::Decode),
                button(text("Encode").horizontal_alignment(alignment::Horizontal::Center))
                    .width(Length::Fill)
                    .on_press(Message::Encode),
                button(image(image::Handle::from_memory(include_bytes!(
                    "../ferris.png"
                ))))
                .width(40)
                .on_press(Message::Crab),
            ],
            text(&self.error).style(Color::from([1.0, 0.0, 0.0])),
            if self.crab {
                image(image::Handle::from_memory(include_bytes!(
                    "../cuddlyferris.png"
                )))
            } else {
                image("")
            },
            // cant be asked to figure out how to include two modules in one if statement (sorry)
            if self.crab {
                text("ferris jumpscare").size(100)
            } else {
                text("")
            }
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
