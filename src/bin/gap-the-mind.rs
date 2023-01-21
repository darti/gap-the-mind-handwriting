use gap_the_mind_handwriting::components::application::GapTheMind;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    GapTheMind::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
