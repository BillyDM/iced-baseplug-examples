#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_specialization)]

use serde::{Serialize, Deserialize};

use baseplug::{
    ProcessContext, Plugin, WindowOpenResult,
};
use raw_window_handle::RawWindowHandle;

mod ui;

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct MonoSynthModel {
        #[model(min = -90.0, max = 3.0)]
        #[parameter(name = "gain", unit = "Decibels",
            gradient = "Power(0.15)")]
        gain: f32
    }
}

impl Default for MonoSynthModel {
    fn default() -> Self {
        Self {
            // "gain" is converted from dB to coefficient in the parameter handling code,
            // so in the model here it's a coeff.
            // -0dB == 1.0
            gain: 1.0
        }
    }
}

struct MonoSynth { }

impl Plugin for MonoSynth {
    const NAME: &'static str = "iced-baseplug monosynth";
    const PRODUCT: &'static str = "iced-baseplug monosynth";
    const VENDOR: &'static str = "spicy plugins & co";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = MonoSynthModel;

    #[inline]
    fn new(_sample_rate: f32, _model: &MonoSynthModel) -> Self {
        Self {}
    }

    #[inline]
    fn process(&mut self, model: &MonoSynthModelProcess, ctx: &mut ProcessContext) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..ctx.nframes {
            output[0][i] = input[0][i] * model.gain[i];
            output[1][i] = input[1][i] * model.gain[i];
        }
    }
}

impl baseplug::PluginUI for MonoSynth {
    type Handle = baseview::WindowHandle;

    fn ui_size() -> (i16, i16) {
        (500, 300)
    }

    fn ui_open(parent: RawWindowHandle) -> WindowOpenResult<Self::Handle> {
        let (width, height) = Self::ui_size();

        let settings = iced_baseview::Settings {
            window: iced_baseview::settings::Window {
                title: String::from("iced-baseplug gain"),
                size: (width as u32, height as u32),
                min_size: None,
                max_size: None,
                resizable: false,
            },
        };

        Ok(iced_baseview::Handler::<ui::MonoSynthUI>::open(settings, Some(parent)))
    }

    fn ui_close(_handle: Self::Handle) {
    }
}

baseplug::vst2!(MonoSynth, b"tAnE");
