use iced_audio::{knob, tick_marks, Knob, LogDBRange, Normal};
use iced_baseview::{
    executor, Align, Color, Column, Command, Container, Element, Length, Row, Rule, Text,
};

mod style;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    GainLeftChanged(Normal),
    GainRightChanged(Normal),
    GainMasterChanged(Normal),
}

pub struct GainUI {
    db_range: LogDBRange,

    gain_left_knob_state: knob::State,
    gain_right_knob_state: knob::State,
    gain_master_knob_state: knob::State,

    left_value_text: String,
    right_value_text: String,
    master_value_text: String,

    db_tick_marks: tick_marks::Group,
}

impl iced_baseview::Application for GainUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let db_range = LogDBRange::new(-90.0, 3.0, 0.8.into());
        (
            Self {
                db_range,

                gain_left_knob_state: knob::State::new(db_range.default_normal_param()),
                gain_right_knob_state: knob::State::new(db_range.default_normal_param()),
                gain_master_knob_state: knob::State::new(db_range.default_normal_param()),

                left_value_text: String::from("0.0"),
                right_value_text: String::from("0.0"),
                master_value_text: String::from("0.0"),

                db_tick_marks: vec![
                    (db_range.map_to_normal(0.0), tick_marks::Tier::One),
                    (Normal::min(), tick_marks::Tier::One),
                    (Normal::max(), tick_marks::Tier::One),
                ]
                .into(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("iced-baseplug-examples gain")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::GainLeftChanged(normal) => {
                let value = self.db_range.unmap_to_value(normal);
                self.left_value_text = db_output_text(value);
            }
            Message::GainRightChanged(normal) => {
                let value = self.db_range.unmap_to_value(normal);
                self.right_value_text = db_output_text(value);
            }
            Message::GainMasterChanged(normal) => {
                let value = self.db_range.unmap_to_value(normal);
                self.master_value_text = db_output_text(value);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Row::new()
            .width(Length::Shrink)
            .align_items(Align::Center)
            .padding(10)
            .spacing(14)
            .push(knob_param(
                "L",
                &mut self.gain_left_knob_state,
                self.left_value_text.as_str(),
                30,
                &self.db_tick_marks,
                Message::GainLeftChanged,
            ))
            .push(knob_param(
                "R",
                &mut self.gain_right_knob_state,
                self.right_value_text.as_str(),
                30,
                &self.db_tick_marks,
                Message::GainRightChanged,
            ))
            .push(Rule::vertical(22).style(style::RuleStyle))
            .push(knob_param(
                "M6",
                &mut self.gain_master_knob_state,
                self.master_value_text.as_str(),
                34,
                &self.db_tick_marks,
                Message::GainMasterChanged,
            ));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn background_color(&self) -> Color {
        style::BACKGROUND_COLOR
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn renderer_settings() -> iced_baseview::renderer::Settings {
        iced_baseview::renderer::Settings {
            default_font: None,
            default_text_size: 20,
            antialiasing: Some(iced_baseview::renderer::Antialiasing::MSAAx8),
            ..iced_baseview::renderer::Settings::default()
        }
    }
}

fn knob_param<'a, Message, F>(
    label: &str,
    knob_state: &'a mut knob::State,
    value_text: &str,
    knob_size: u16,
    tick_marks: &'a tick_marks::Group,
    on_change: F,
) -> Column<'a, Message>
where
    F: 'static + Fn(Normal) -> Message,
    Message: 'a,
{
    let knob = Knob::new(knob_state, on_change)
        .size(Length::Units(knob_size))
        .style(style::KnobStyle)
        .tick_marks(tick_marks);

    Column::new()
        .width(Length::Units(38))
        .align_items(Align::Center)
        .spacing(12)
        .padding(0)
        .push(Text::new(label).size(14).color(style::FONT_COLOR))
        .push(knob)
        .push(Text::new(value_text).size(14).color(style::FONT_COLOR))
}

fn db_output_text(value: f32) -> String {
    format!("{:.1}", value)
}
