es, this is quite solid overall and already feels idiomatic for a mid-sized Rust project that wants a clean public API for the library while keeping the CLI as a thin layer on top. You've done the hard part well: separating concerns into a workspace, making the library reusable, and giving RenderOptions a nice ergonomic surface.
Here are the main strengths and some targeted suggestions for polish.
Workspace & Crate Structure
This is a very common and recommended pattern for tools that want both a good library and a polished CLI.

lib/ (named px2ansi) contains the pure domain logic → excellent.
cli/ (the binary) depends on the lib via path dependency → correct.
Workspace with shared [workspace.package], [workspace.dependencies], lints, and release profile → very clean and DRY.

Minor notes:

In [workspace.dependencies] you have both px2ansi = { path = "lib" } and px2ansi-rs = { path = "cli" }. The second one is unnecessary unless something inside cli re-exports the binary or you have weird intra-workspace cycles. Most people just depend on px2ansi from the CLI crate.
Consider renaming the CLI crate to px2ansi-cli (and update the binary name if you want cargo install px2ansi-cli). This is a popular convention (see ripgrep → ripgrep, but many others like bat vs potential lib, or fd).

The structure aligns well with Rust community advice: keep the binary thin, put real logic in the lib.
Library API (RenderOptions and friends)
This is the most visible part, and it's already good.
Strengths:

RenderOptionsBuilder with fluent .preset().density().width()... .build() is exactly the right level of ergonomics for something with presets + overrides.
RenderOptions::with_preset(...) and RenderOptions::builder() provide both quick and flexible paths.
From<RenderStylePreset> for the preset baseline is nice.
prepare_image() and render_centered() are clear, focused methods.
no_color() as a consuming method is a cute touch (though see below).

Areas for improvement / polish:

Builder design
Your RenderOptionsBuilder is mostly consuming (mut self), which is fine, but many Rust libraries prefer the non-consuming style for builders (methods take &mut self and return &mut Self). It makes chaining after conditionals or in loops slightly nicer and avoids unnecessary moves.
Example:Rustpub fn preset(mut self, preset: RenderStylePreset) -> Self { ... }  // current
// vs
pub fn preset(&mut self, preset: RenderStylePreset) -> &mut Self { ... }Both are idiomatic; the &mut version is a bit more common in larger builders (see clap::Command, reqwest, etc.). Your current version works fine though.
Default + overrides flow
The way you do "start with preset or global default, then apply explicit overrides" is reasonable, but it can be a bit subtle. Some people prefer the builder to always start from Default and have the preset just be another override method that sets multiple fields at once. Your approach is clearer for "presets as strong defaults", so it's okay.
Small API nits
no_color(mut self) is unusual because most other methods are getters or non-consuming. Consider making it a builder method only (builder.color(false)) or add a RenderOptions::no_color(self) -> Self that is clearly consuming.
render_centered takes &DynamicImage and does the resize inside. That's convenient, but consider also exposing a lower-level render(&self, img: &DynamicImage, writer: &mut W) that assumes the image is already the right size. Power users often want to control resizing themselves.
CharsetMode::from_str uses anyhow::bail!. For a library, it's usually better to define your own error type with thiserror (or at least a custom enum) so users can match on "invalid charset" specifically. anyhow is great in the CLI and in main, but libraries should expose structured errors when the error is part of the public contract.

Types module
CharsetMode, Density, RenderStyle living in render::types is fine. You could also put them directly in render or even re-export them at the crate root (pub use crate::render::types::*;) if users will interact with them a lot. For now it's okay.
Other library polish
Add more documentation examples on the main types and RenderOptions::builder().
Consider whether RenderStyle should be public or if most of its fields can be hidden behind methods (right now full and density are public).
get_terminal_size is used in render_centered — make sure it's re-exported or easy to mock for tests.


CLI Layer (cli/src/main.rs)
This is also quite clean.
Good things:

Thin main() that parses, loads config, builds a Command enum, then delegates → excellent separation.
ResolvedOptions to merge CLI + config is a solid pattern.
build_render_options helper is nice.
Special-casing completions early is correct.

Suggestions:

The Command enum + handle_command + per-command structs (ConvertCmd, etc.) is a good way to avoid a god main. Some people go further and make each command its own module with a run(&self) -> Result<()> method — you're basically there.
confy for config is fine for simple cases.
Consider adding --help examples or using clap's after_help / long_about more aggressively.

General / Minor Things

Edition 2024 — nice, you're forward-looking.
Lints are aggressive (pedantic + nursery) with some sensible allows — good.
Release profile with LTO + panic=abort + strip is perfect for a CLI tool.
Dependencies look reasonable. colored is okay but some people prefer owo-colors or anstyle for more control; not worth changing now.
Error handling: anyhow everywhere in the lib is acceptable for a small tool, but as the library grows, introducing a px2ansi::Error enum (via thiserror) for expected failures (invalid charset, image decode errors you want to surface nicely, etc.) is the more "library-like" approach.

Overall Verdict
Yes, this looks idiomatic and the library feels well-structured for what it does (image → terminal art with multiple styles). The public API for RenderOptions is already nicer than many similar tools.
With the small tweaks above (especially error types, maybe builder style, and exposing a lower-level render path), it would feel very professional and "crates.io-ready" if you ever want to publish the lib separately.
Would you like me to sketch a refactored version of RenderOptions / RenderOptionsBuilder with the &mut self builder style, or suggestions for a custom error type? Or anything else specific (tests, docs, etc.)?
