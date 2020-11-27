# iced baseplug examples
Simple examples to demonstrate full-stack Rust audio plugin dev with [`baseplug`] and [`iced_audio`]

WIP

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
[`iced_audio`]: https://github.com/BillyDM/iced_audio
