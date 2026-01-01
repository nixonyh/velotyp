<div align="center">

# Velotyp

**An integration to render [Typst](https://typst.app/home) with [Vello](https://vello.dev).**

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23vello-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/197075-vello)
[![dependency status](https://deps.rs/repo/github/linebender/velotyp/status.svg)](https://deps.rs/repo/github/linebender/velotyp)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![vello version](https://img.shields.io/badge/vello-v0.6.0-purple.svg)](https://crates.io/crates/vello)\
[![Crates.io](https://img.shields.io/crates/v/velotyp.svg)](https://crates.io/crates/velotyp)
[![Docs](https://docs.rs/velotyp/badge.svg)](https://docs.rs/velotyp)
[![Build status](https://github.com/linebender/velotyp/workflows/CI/badge.svg)](https://github.com/linebender/velotyp/actions)

</div>

## Version compatibility

| velotyp   | vello | typst |
| --------- | ----- | ----- |
| 0.1, main | 0.6   | 0.14  |

<!-- ## Usage -->

<!-- Velotyp makes it simple to encode Lottie as a [`vello::Scene`](https://docs.rs/vello/*/vello/struct.Scene.html). -->

<!-- ```rust -->
<!-- // Parse your lottie file -->
<!-- let lottie = include_str!("../lottie.json"); -->
<!-- let composition = velotyp::Composition::from_str(lottie).expect("valid file"); -->

<!-- // Render to a scene -->
<!-- let mut new_scene = vello::Scene::new(); -->

<!-- // Render to a scene! -->
<!-- let mut renderer = velotyp::Renderer::new(); -->
<!-- let frame = 0.0; // Arbitrary number chosen. Ensure it's a valid frame! -->
<!-- let transform = vello::kurbo::Affine::IDENTITY; -->
<!-- let alpha = 1.0; -->
<!-- renderer.render(&composition, frame, transform, alpha, &mut new_scene); -->
<!-- ``` -->

<!-- ## Examples -->

<!-- ### Cross platform (Winit) -->

<!-- ```shell -->
<!-- cargo run -p with_winit -->
<!-- ``` -->

<!-- You can also load an entire folder or individual files. -->

<!-- ```shell -->
<!-- cargo run -p with_winit -- examples/assets -->
<!-- ``` -->

<!-- ### Web platform -->

<!-- Because Vello relies heavily on compute shaders, we rely on the emerging WebGPU standard to run on the web. -->
<!-- Until browser support becomes widespread, it will probably be necessary to use development browser versions (e.g. Chrome Canary) and explicitly enable WebGPU. -->

<!-- This uses [`cargo-run-wasm`](https://github.com/rukai/cargo-run-wasm) to build the example for web, and host a local server for it -->

<!-- ```shell -->
<!-- # Make sure the Rust toolchain supports the wasm32 target -->
<!-- rustup target add wasm32-unknown-unknown -->

<!-- # The binary name must also be explicitly provided as it differs from the package name -->
<!-- cargo run_wasm -p with_winit --bin with_winit_bin -->
<!-- ``` -->

<!-- There is also a web demo [available here](https://linebender.github.io/velotyp) on supporting web browsers. -->

<!-- > [!WARNING] -->
<!-- > The web is not currently a primary target for Vello, and WebGPU implementations are incomplete, so you might run into issues running this example. -->

## Minimum supported Rust Version (MSRV)

This version of Velotyp has been verified to compile with **Rust 1.89** and later.

Future versions of Velotyp might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of Velotyp's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```

</details>

## Community

Discussion of Velotyp development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically the [#vello channel](https://xi.zulipchat.com/#narrow/channel/197075-vello). All public content can be read without logging in.

Contributions are welcome by pull request. The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

