vergen\_pretty - Rust

if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-6b053e98.ttf.woff2,FiraSans-Regular-0fe48ade.woff2,FiraSans-Medium-e1aa3f0a.woff2,SourceCodePro-Regular-8badfe75.ttf.woff2,SourceCodePro-Semibold-aa29a496.ttf.woff2".split(",").map(f=>\`<link rel="preload" as="font" type="font/woff2" crossorigin href="/-/rustdoc.static/${f}">\`).join(""))

<link rel="stylesheet" href="/-/rustdoc.static/noscript-893ab5e7.css">

(function() { function applyTheme(theme) { if (theme) { document.documentElement.dataset.docsRsTheme = theme; } } window.addEventListener("storage", ev => { if (ev.key === "rustdoc-theme") { applyTheme(ev.newValue); } }); // see ./storage-change-detection.html for details window.addEventListener("message", ev => { if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") { applyTheme(ev.data.storage.value); } }); applyTheme(window.localStorage.getItem("rustdoc-theme")); })();

 [Docs.rs](/)

{ "name": "vergen-pretty", "version": "0.3.9" }

*   [vergen-pretty-0.3.9](# "Output vergen information in a formatted manner")

    *   vergen-pretty 0.3.9
    *   [Permalink](/vergen-pretty/0.3.9/vergen_pretty/index.html "Get a link to this specific version")
    *   [Docs.rs crate page](/crate/vergen-pretty/latest "See vergen-pretty in docs.rs")
    *   [MIT OR Apache-2.0](/crate/vergen-pretty/latest)

    *   Links
    *   [Homepage](https://github.com/rustyhorde/vergen)
    *   [Repository](https://github.com/rustyhorde/vergen)
    *   [crates.io](https://crates.io/crates/vergen-pretty "See vergen-pretty in crates.io")
    *   [Source](/crate/vergen-pretty/latest/source/ "Browse source of vergen-pretty-0.3.9")

    *   Owners
    *   [CraZySacX](https://crates.io/users/CraZySacX)

    *   Dependencies
    *   *   [anyhow ^1.0.95 _normal_](/anyhow/^1.0.95)
        *   [console ^0.15.10 _normal_ _optional_](/console/^0.15.10)
        *   [convert\_case ^0.6.0 _normal_](/convert_case/^0.6.0)
        *   [derive\_builder ^0.20.2 _normal_](/derive_builder/^0.20.2)
        *   [lazy\_static ^1.5.0 _normal_ _optional_](/lazy_static/^1.5.0)
        *   [rand ^0.8.5 _normal_ _optional_](/rand/^0.8.5)
        *   [serde ^1.0.217 _normal_ _optional_](/serde/^1.0.217)
        *   [tracing ^0.1.41 _normal_ _optional_](/tracing/^0.1.41)
        *   [lazy\_static ^1.5.0 _dev_](/lazy_static/^1.5.0)
        *   [regex ^1.11.1 _dev_](/regex/^1.11.1)
        *   [serde\_json ^1.0.135 _dev_](/serde_json/^1.0.135)
        *   [tracing-subscriber ^0.3.19 _dev_](/tracing-subscriber/^0.3.19)
        *   [anyhow ^1.0.95 _build_](/anyhow/^1.0.95)
        *   [rustversion ^1.0.19 _build_](/rustversion/^1.0.19)
        *   [vergen-gix ^1.0.5 _build_ _optional_](/vergen-gix/^1.0.5)




    *   Versions
    *

    *   [**100%** of the crate is documented](/crate/vergen-pretty/latest)







*   [Platform](#)
    *   [i686-pc-windows-msvc](/crate/vergen-pretty/latest/target-redirect/i686-pc-windows-msvc/vergen_pretty/index.html)
    *   [i686-unknown-linux-gnu](/crate/vergen-pretty/latest/target-redirect/i686-unknown-linux-gnu/vergen_pretty/index.html)
    *   [x86\_64-apple-darwin](/crate/vergen-pretty/latest/target-redirect/x86_64-apple-darwin/vergen_pretty/index.html)
    *   [x86\_64-pc-windows-msvc](/crate/vergen-pretty/latest/target-redirect/x86_64-pc-windows-msvc/vergen_pretty/index.html)
    *   [x86\_64-unknown-linux-gnu](/crate/vergen-pretty/latest/target-redirect/x86_64-unknown-linux-gnu/vergen_pretty/index.html)
*   [Feature flags](/crate/vergen-pretty/latest/features "Browse available feature flags of vergen-pretty-0.3.9")

*   [Rust](#)
    *   [About docs.rs](/about)
    *   [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)
    *   [Rust website](https://www.rust-lang.org/)
    *   [The Book](https://doc.rust-lang.org/book/)
    *   [Standard Library API Reference](https://doc.rust-lang.org/std/)
    *   [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    *   [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
    *   [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)



## [vergen\_pretty](../vergen_pretty/index.html)0.3.9

*   [All Items](all.html)

### [Sections](#)

*   [Example](#example "Example")
*   [Example](#example-1 "Example")
    *   [Features](#features "Features")

### [Crate Items](#macros)

*   [Macros](#macros "Macros")
*   [Structs](#structs "Structs")
*   [Enums](#enums "Enums")
*   [Functions](#functions "Functions")
*   [Type Aliases](#types "Type Aliases")

# Crate vergen\_prettyCopy item path

[Source](../src/vergen_pretty/lib.rs.html#9-479)

Expand description

`vergen-pretty` - A pretty printer for vergen environment variables

Because `cargo` doesn’t pass compile time environment variables to dependencies, the [`vergen_pretty_env`](macro.vergen_pretty_env.html "macro vergen_pretty::vergen_pretty_env") macro embeds a map of all the possible `vergen` environment variables with [`option_env!`](https://doc.rust-lang.org/nightly/core/macro.option_env.html "macro core::option_env"). Values not set in by your `build.rs` are skipped when pretty-printing the output.

## [§](#example)Example

```
let mut stdout = vec![];
PrettyBuilder::default()
    .env(vergen_pretty_env!())
    .build()?
    .display(&mut stdout)?;
assert!(!stdout.is_empty());
```

See the [`Pretty`](struct.Pretty.html "struct vergen_pretty::Pretty") documentation for more examples

If you enable the header feature, you can also use the [`header()`](fn.header.html "fn vergen_pretty::header") function with the associated [`Config`](struct.Config.html "struct vergen_pretty::Config") as a convenience wrapper around [`Pretty`](struct.Pretty.html "struct vergen_pretty::Pretty").

## [§](#example-1)Example

```
let mut buf = vec![];
let config = ConfigBuilder::default()
    .style(Style::new().green())
    .prefix("HEADER_PREFIX")
    .env(vergen_pretty_env!())
    .suffix("HEADER_SUFFIX")
    .build()?;
assert!(header(&config, Some(&mut buf)).is_ok());
assert!(!buf.is_empty());
```

### [§](#features)Features

`vergen-pretty` has two feature toggles allowing you to customize your output. No features are enabled by default.
You **must** specifically enable the features you wish to use.

FeatureEnables

color

Colorize output, allow configuration of coloring via [`console`](https://docs.rs/console/0.15.10/x86_64-unknown-linux-gnu/console/index.html "mod console")

header

Generate pretty printed header output based on the given [`Config`](struct.Config.html "struct vergen_pretty::Config")

trace

Enable support for [`tracing`](https://docs.rs/tracing/latest/tracing/) output

## Macros[§](#macros)

*   [vergen\_pretty\_env](macro.vergen_pretty_env.html "macro vergen_pretty::vergen_pretty_env")

    Used to initialize `env` in [`PrettyBuilder`](struct.PrettyBuilder.html "struct vergen_pretty::PrettyBuilder")


## Structs[§](#structs)

*   [Config](struct.Config.html "struct vergen_pretty::Config")`header`

    Convenience configuration around [`crate::Pretty`](struct.Pretty.html "struct vergen_pretty::Pretty") to ease output generation.

*   [ConfigBuilder](struct.ConfigBuilder.html "struct vergen_pretty::ConfigBuilder")`header`

    Builder for [`Config`](struct.Config.html).

*   [Level](struct.Level.html "struct vergen_pretty::Level")`trace`

    Describes the level of verbosity of a span or event.

*   [Prefix](struct.Prefix.html "struct vergen_pretty::Prefix")

    Configure prefix output for [`PrettyBuilder`](struct.PrettyBuilder.html "struct vergen_pretty::PrettyBuilder")

*   [PrefixBuilder](struct.PrefixBuilder.html "struct vergen_pretty::PrefixBuilder")

    Builder for [`Prefix`](struct.Prefix.html).

*   [Pretty](struct.Pretty.html "struct vergen_pretty::Pretty")

    Configuration for `vergen` environment variable pretty printing

*   [PrettyBuilder](struct.PrettyBuilder.html "struct vergen_pretty::PrettyBuilder")

    Builder for [`Pretty`](struct.Pretty.html).

*   [Style](struct.Style.html "struct vergen_pretty::Style")`color`

    A stored style that can be applied.

*   [Suffix](struct.Suffix.html "struct vergen_pretty::Suffix")

    Configure suffix output for [`PrettyBuilder`](struct.PrettyBuilder.html "struct vergen_pretty::PrettyBuilder")

*   [SuffixBuilder](struct.SuffixBuilder.html "struct vergen_pretty::SuffixBuilder")

    Builder for [`Suffix`](struct.Suffix.html).


## Enums[§](#enums)

*   [PrettyBuilderError](enum.PrettyBuilderError.html "enum vergen_pretty::PrettyBuilderError")

    Error type for PrettyBuilder


## Functions[§](#functions)

*   [header](fn.header.html "fn vergen_pretty::header")`header`

    Generate a pretty header based off your emitted `vergen` variables.


## Type Aliases[§](#types)

*   [Env](type.Env.html "type vergen_pretty::Env")`header`

    Environment tree type alias
