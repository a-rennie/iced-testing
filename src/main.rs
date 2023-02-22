use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings, window, Color};
use ascii_crypt::{encode, decode};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (500,500),
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
}

#[derive(Debug, Clone)]
enum Message {
    Encode,
    Decode,
    InputChanged(String),
}

impl Sandbox for Crypt {
    type Message = Message;

    fn new() -> Self {
        Self { value: String::new(), error: String::new() }
    }

    fn title(&self) -> String {
        String::from("Encoder/Decoder")
    }

    fn update(&mut self, message: Message) {
            match message {
                Message::Encode => {
                    self.value = match encode(&self.value){
                        Ok(message) => message,
                        Err(err) => {
                            self.error = err;
                            String::from("")
                        }
                    };
                }
                Message::Decode => {
                    self.value = match decode(&self.value){
                        Ok(message) => message,
                        Err(err) => {
                            self.error = err;
                            String::from("")
                        }
                    };
                }
                Message::InputChanged(str) => self.value = str
            }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("Type something...", &self.value,  Message::InputChanged),
            row![button("Decode").on_press(Message::Decode), button("Encode").on_press(Message::Encode)],
            text(&self.error).style(Color::from([1.0, 0.0, 0.0]))
        ]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}
