<!-- TOC -->
# Table of contents

1. [Table of contents](#table-of-contents)
1. [Requirements](#requirements)
1. [Get help from the CLI](#get-help-from-the-cli)
1. [If there's no output for a while](#if-there's-no-output-for-a-while)
<!-- TOC -->

# Requirements

1. The [Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) specified in `rust-toolchain.toml`.
1. [cargo-run-bin](https://crates.io/crates/cargo-run-bin).
1. [Node.js](https://nodejs.org) and an `$ npm install`.

# Get help from the CLI

```
$ cargo run -- --help
```

# If there's no output for a while

That may be [because of `cargo-run-bin`](https://github.com/dustinblackman/cargo-run-bin/issues/2).
It's compiling executable depenedencies in the background and does not pass the output through.

If you're not certain whether compilation is in progress, consider looking for a `rustc` process.

# Architecture

1. The website content is generated at build time, except for the calendars.
1. The calendars are generated at runtime, except for the contents of the calendar event element.

## Build time generated content

1. All content is generated by components.
1. A component is a type that implements `maud::Render`.
1. A component `Foo` and its implementations are in a module `crate::components::foo`.
1. The `crate::components` module re-exports all the components.
1. Components are never imported directly into scope.
   Instead, the `crate::components` module is brought into scope.
   A component `Foo` is referred to as `components::Foo`.
1. Component fields are `pub(crate)`. No constructor methods.
   An example of how a component is used:

   ```rust
   html! {
     (components::Foo {
       a: 1
     })
   }
   ```
1. Some types that implement `maud::Render` are not components.
   This is because they are used as HTML attribute values.
   The `maud::Render` trait is used for attribute values, as well.
