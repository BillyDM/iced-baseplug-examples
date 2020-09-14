use iced_native::{
    Align, Color, Column, Command, Container, Element, Length, Text,
};

#[derive(Debug, Copy, Clone)]
pub enum Message {

}

pub struct MonoSynthUI {

}

impl iced_baseview::Application for MonoSynthUI {
    type AudioToGuiMessage = ();
    type Message = Message;

    fn new() -> Self {
        Self {

        }
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, iced_baseview::Renderer> {
        let content = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .padding(20)
            .spacing(20)
            .push(Text::new("Hello World!"));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn background_color() -> Color {
        Color::WHITE
    }

    fn compositor_settings() -> iced_baseview::CompositorSettings {
        iced_baseview::CompositorSettings {
            default_font: None,
            default_text_size: 20,
            antialiasing: Some(iced_baseview::Antialiasing::MSAAx8),
            ..iced_baseview::CompositorSettings::default()
        }
    }
}