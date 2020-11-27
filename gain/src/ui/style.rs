use iced_audio::{knob, tick_marks};
use iced_baseview::{rule, Color};

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
impl KnobStyle {
    const ACTIVE_STYLE: knob::CircleStyle = knob::CircleStyle {
        color: KNOB_COLOR,
        border_width: 1.0,
        border_color: KNOB_BORDER_COLOR,
        notch: knob::NotchShape::Line(knob::LineNotch {
            color: KNOB_NOTCH_COLOR,
            width: knob::StyleLength::Units(3.5),
            length: knob::StyleLength::Scaled(0.3),
            offset: knob::StyleLength::Units(0.0),
            cap: knob::LineCap::Butt,
        }),
    };
}
impl knob::StyleSheet for KnobStyle {
    fn active(&self) -> knob::Style {
        knob::Style::Circle(Self::ACTIVE_STYLE)
    }

    fn hovered(&self) -> knob::Style {
        knob::Style::Circle(knob::CircleStyle {
            color: KNOB_HOVERED_COLOR,
            ..Self::ACTIVE_STYLE
        })
    }

    fn dragging(&self) -> knob::Style {
        self.hovered()
    }

    fn tick_marks_style(&self) -> Option<knob::TickMarksStyle> {
        Some(knob::TickMarksStyle {
            style: tick_marks::Style {
                tier_1: tick_marks::Shape::Line {
                    length: 3.0,
                    width: 2.0,
                    color: TICK_MARK_COLOR,
                },
                tier_2: tick_marks::Shape::Line {
                    length: 2.0,
                    width: 2.0,
                    color: TICK_MARK_COLOR,
                },
                tier_3: tick_marks::Shape::Line {
                    length: 2.0,
                    width: 2.0,
                    color: TICK_MARK_COLOR,
                },
            },
            offset: 3.5,
        })
    }
}

pub struct RuleStyle;
impl rule::StyleSheet for RuleStyle {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: RULE_COLOR,
            width: 1,
            radius: 0.0,
            fill_mode: rule::FillMode::Percent(50.0),
        }
    }
}
