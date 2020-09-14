use iced_native::{
    Align, Color, Column, Command, Container, Element, Length, Text, Row, Rule,
};

use iced_audio::{knob, Knob, LogDBRange, TickMarkGroup, TickMark, TickMarkTier, Normal};

mod style;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    ParamChanged(ParamID),
}

#[derive(Debug, Copy, Clone)]
pub enum ParamID {
    GainLeft,
    GainRight,
    GainMaster,
}

pub struct GainUI {
    db_range: LogDBRange,

    gain_left_knob_state: knob::State<ParamID>,
    gain_right_knob_state: knob::State<ParamID>,
    gain_master_knob_state: knob::State<ParamID>,

    left_value_text: String,
    right_value_text: String,
    master_value_text: String,

    db_tick_marks: TickMarkGroup,
}

impl iced_baseview::Application for GainUI {
    type AudioToGuiMessage = ();
    type Message = Message;

    fn new() -> Self {
        let db_range = LogDBRange::new(-90.0, 3.0, 0.8.into());

        Self {
            db_range,

            gain_left_knob_state: knob::State::new(
                db_range.create_param_default(ParamID::GainLeft),
            ),
            gain_right_knob_state: knob::State::new(
                db_range.create_param_default(ParamID::GainRight),
            ),
            gain_master_knob_state: knob::State::new(
                db_range.create_param_default(ParamID::GainMaster),
            ),

            left_value_text: String::from("0.0"),
            right_value_text: String::from("0.0"),
            master_value_text: String::from("0.0"),

            db_tick_marks: vec![
                TickMark {
                    position: db_range.to_normal(0.0),
                    tier: TickMarkTier::One,
                },

                TickMark {
                    position: Normal::max(),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: Normal::min(),
                    tier: TickMarkTier::Two,
                },
            ].into(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ParamChanged(param) => {
                match param {
                    ParamID::GainLeft => {
                        let value = self.db_range.to_value(*self.gain_left_knob_state.normal());
                        self.left_value_text = db_output_text(value);
                    }
                    ParamID::GainRight => {
                        let value = self.db_range.to_value(*self.gain_right_knob_state.normal());
                        self.right_value_text = db_output_text(value);
                    }
                    ParamID::GainMaster => {
                        let value = self.db_range.to_value(*self.gain_master_knob_state.normal());
                        self.master_value_text = db_output_text(value);
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, iced_baseview::Renderer> {
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
            ))
            .push(knob_param(
                "R",
                &mut self.gain_right_knob_state,
                self.right_value_text.as_str(),
                30,
                &self.db_tick_marks,
            ))
            .push(Rule::vertical(22).style(style::RuleStyle))
            .push(knob_param(
                "M",
                &mut self.gain_master_knob_state,
                self.master_value_text.as_str(),
                34,
                &self.db_tick_marks,
            ));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn background_color() -> Color {
        style::BACKGROUND_COLOR
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

fn knob_param<'a>(
    label: &str,
    knob_state: &'a mut knob::State<ParamID>,
    value_text: &str,
    knob_size: u16,
    tick_marks: &'a TickMarkGroup,
) -> Column<'a, Message, iced_baseview::Renderer> {
    let knob = Knob::new(knob_state, Message::ParamChanged)
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