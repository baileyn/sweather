use dotenv::dotenv;
use iced::{
    text_input, Align, Column, Container, Element, HorizontalAlignment, Length, Sandbox, Settings,
    Text, TextInput,
};
use log::*;
use pretty_env_logger::*;
use weatherstack::blocking::*;

#[derive(Debug)]
struct Sweather {
    api: WeatherStack,
    state: State,
}

#[derive(Debug, Default)]
struct State {
    location_input: text_input::State,
    location_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    LookupWeather,
}

impl Sandbox for Sweather {
    type Message = Message;

    fn new() -> Self {
        Self {
            api: WeatherStack::new(
                std::env::var("API_ACCESS_KEY").expect("Unable to obtain API_ACCESS_KEY"),
            ),
            state: State::default(),
        }
    }

    fn title(&self) -> String {
        "Sweather".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(value) => {
                self.state.location_value = value;
            }
            Message::LookupWeather => {
                debug!("Looking up weather for: {}", self.state.location_value);
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("SWEATHER").size(50).color([0.5, 0.5, 0.5]);

        let input = TextInput::new(
            &mut self.state.location_input,
            "Location",
            &self.state.location_value,
            Message::InputChanged,
        )
        .padding(15)
        .on_submit(Message::LookupWeather);

        let content = Column::new()
            .width(Length::Units(800))
            .align_items(Align::Center)
            .spacing(20)
            .push(title)
            .push(input);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .into()
    }
}

fn main() -> iced::Result {
    dotenv().ok();
    pretty_env_logger::init();

    Sweather::run(Settings::default())
}
