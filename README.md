# Verlet solver

A Verlet solver based on
[this](https://youtu.be/lS_qeBy3aQI) and
[this](https://youtu.be/9IULfQH7E90) video, test it
[here](https://ghtghoster.github.io/mq-verlet/).
The purpose of this project is to familiarize myself with simple physics
simulations, shaders, and using `egui` and `macroquad` crates.

Presets are made with a fullscreen or near fullscreen window at FHD resolution in mind.

This code is undocumented as it was an exploratory project with me having
very little to no idea of what I was doing as I was doing it. Even so,
the code should be structured relatively nicely, hopefully enough so
that it's readable and undestandable without excessive headaches.

More interesting crates to look out for in the future:
- [`quad-storage`](https://crates.io/crates/quad-storage)
- [`quad-url`](https://crates.io/crates/quad-url)

Be advised that `egui-miniquad`'s dependency `quad-url` relies on an old,
[vulnerable](https://github.com/advisories/GHSA-m589-mv4q-p7rj)
version of `webbrowser` and neither me nor GitHub's Dependabot can fix it.
This isn't actually used in code anywhere but be careful.

## Limits and issues:

Approximate object limits before FPS drops (mileage may vary):
- Naive (every object against every other object): 1600
- Cellularized (matrix/cell-based optimization): 3300
- Optimized heap usage: 5000

Few glaring issue of the simulation:
- The optimization cell block size has to be quite a bit larger than the object
  given a homogenous obj size due to "popcorn effect"
- The simulation still freaks out at large quantities of objects moving
- Not running quite as fast as I'd hoped
- Only supports circles
- Relying on shaders and passing in uniforms to render anything but monochrome
  circles is slow
- When running this natively without target SFPS enabled, at very high
  frame-ratesthe simulation freaks out. (Pressumably because of f32 rounding error)

## Plans and features that didn't make it

- resistance
- making wasm work for android
- time warp thingy, including complete stop (adjust frame_time before
  passing to update)
- make everything f64 (or generic?) and compare performance
- spawned from this: 3d version
- the fire is about as good as I can get it, maybe try removing temperature
  based on how much the object traveled since last frame
- learn how to use shaders more effectively (water shader in the book, passing
  in whole textures etc) - screen reading shaders
- auto shaking (with looping over stuff and bpm/settable delay per shake)
- mixer (cw or ccw shake timed accordingly)
- if last sim frame time < target frame time: disable target frame time

## Instructions and dependencies:

All scripts listed below are compatible with default Windows installation of
PowerShell (v6+ not required) as well as bash for Linux (scripts are polyglot)

(The bash portion of the polyglot scripts is untested, use with caution
and please report back with results or a pull request)

### [`rename.ps1`](rename.ps1)
This script changes the internal name of the project in the files
[`src\main.rs`](src\main.rs),
[`Cargo.toml`](Cargo.toml), and
[`index.html`](index.html)
to match the name of the repository, and allows `cargo` to work correctly.

(This is only necessary to run once after a repository was first created with the
[`mq-wbg-template`](https://github.com/GhtGhoster/mq-wbg-template) template.) 

### [`setup.ps1`](setup.ps1)
This script installs `wasm-bindgen-cli`, `basic-http-server`
and adds `wasm32-unknown-unknown` to possible compilation targets.

(This is only necessary to run once on a single computer as the effects
of this script are global.)

### [`build.ps1`](build.ps1)
This script builds the project for the `wasm32-unknown-unknown` target in
`--release` mode, generates WASM bindings, and patches the generated JavaScript
file. It also moves the relevant files to their appropriate directories
in preparation for running the project on a local server or on GitHub Pages.

### [`run.ps1`](run.ps1)
This script hosts the built project on a local `basic-http-server`
server and opens a browser at its location.

(One does not need to restart the server after building the project again,
reloading the webpage in the browser is sufficent.)

(This is necessary over just opening the [`index.html`](index.html)
file in your browser so that the required resources load properly.)

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
