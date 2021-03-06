# iced baseplug examples
Simple examples to demonstrate full-stack Rust audio plugin dev with [`baseplug`] and [`iced_audio`]

WIP (The GUI knobs do nothing currently. This will be solved once [`baseplug`] gets official functionality for syncing parameters between backend & GUI.)

<div align="center">
    <img src="/screenshots/gain.png">
</div>

## To build and run:
1. Install dependencies:
    * Ubuntu: `sudo apt install libx11-xcb-dev libxcb-dri2-0-dev libgl1-mesa-dev libxcb-icccm4-dev libxcursor-dev`
2. Build example with `nightly` Rust toolchain:
    * `cargo +nightly build --package gain --release`
3. Load the resulting `libgain.so` (`libgain.dll` on Windows) file into a VST2 Host.

[`baseplug`]: https://github.com/wrl/baseplug
[`iced_audio`]: https://github.com/iced-rs/iced_audio
