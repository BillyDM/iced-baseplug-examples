use iced_graphics::{Color, rule};
use iced_audio::{
    knob,
};

pub const BACKGROUND_COLOR: Color = Color::from_rgb(
    0x16 as f32 / 255.0,
    0x1d as f32 / 255.0,
    0x20 as f32 / 255.0,
);

pub const KNOB_COLOR: Color = Color::from_rgb(
    0x2a as f32 / 255.0,
    0xa1 as f32 / 255.0,
    0x31 as f32 / 255.0,
);

pub const KNOB_HOVERED_COLOR: Color = Color::from_rgb(
    0x29 as f32 / 255.0,
    0xa8 as f32 / 255.0,
    0x30 as f32 / 255.0,
);

pub const KNOB_BORDER_COLOR: Color = Color::from_rgb(
    0x80 as f32 / 255.0,
    0xe6 as f32 / 255.0,
    0x8b as f32 / 255.0,
);

pub const KNOB_NOTCH_COLOR: Color = Color::from_rgb(
    0xc6 as f32 / 255.0,
    0xff as f32 / 255.0,
    0xf9 as f32 / 255.0,
);

pub const TICK_MARK_COLOR: Color = Color::from_rgba(
    0xe3 as f32 / 255.0,
    0xff as f32 / 255.0,
    0xfc as f32 / 255.0,
    0x55 as f32 / 255.0,
);

pub const RULE_COLOR: Color = Color::from_rgba(
    0xe3 as f32 / 255.0,
    0xff as f32 / 255.0,
    0xfc as f32 / 255.0,
    0x25 as f32 / 255.0,
);

pub const FONT_COLOR: Color = Color::from_rgb(
    0xe3 as f32 / 255.0,
    0xff as f32 / 255.0,
    0xfc as f32 / 255.0,
);

pub struct KnobStyle;
impl knob::StyleSheet for KnobStyle {
    fn active(&self) -> knob::Style {
        knob::Style::ClassicLine(knob::ClassicLineStyle {
            color: KNOB_COLOR,
            border_width: 1,
            border_color: KNOB_BORDER_COLOR,
            notch_color: KNOB_NOTCH_COLOR,
            notch_width: 3.5,
            notch_scale: 0.6.into(),
            notch_offset: 0.0.into(),
        })
    }

    fn hovered(&self) -> knob::Style {
        if let knob::Style::ClassicLine(active) = self.active() {
            knob::Style::ClassicLine(knob::ClassicLineStyle {
                color: KNOB_HOVERED_COLOR,
                ..active
            })
        } else { self.active() }
    }

    fn dragging(&self) -> knob::Style {
        self.hovered()
    }

    fn tick_mark_style(&self) -> Option<knob::TickMarkStyle> {
        Some(knob::TickMarkStyle::Line(knob::LineTickMarkStyle {
            length_tier_1: 3.5,
            length_tier_2: 2.5,
            length_tier_3: 2.5,

            width_tier_1: 2.0,
            width_tier_2: 1.75,
            width_tier_3: 1.75,

            color_tier_1: TICK_MARK_COLOR,
            color_tier_2: TICK_MARK_COLOR,
            color_tier_3: TICK_MARK_COLOR,

            offset: 3.5,
        }))
    }
}

pub struct RuleStyle;
impl rule::StyleSheet for RuleStyle {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: RULE_COLOR,
            width: 1,
            radius: 0,
            fill_mode: rule::FillMode::Percent(50.0),
        }
    }
}