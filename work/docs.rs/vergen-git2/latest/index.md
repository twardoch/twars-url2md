vergen\_git2 - Rust

if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-6b053e98.ttf.woff2,FiraSans-Regular-0fe48ade.woff2,FiraSans-Medium-e1aa3f0a.woff2,SourceCodePro-Regular-8badfe75.ttf.woff2,SourceCodePro-Semibold-aa29a496.ttf.woff2".split(",").map(f=>\`<link rel="preload" as="font" type="font/woff2" crossorigin href="/-/rustdoc.static/${f}">\`).join(""))

<link rel="stylesheet" href="/-/rustdoc.static/noscript-893ab5e7.css">

(function() { function applyTheme(theme) { if (theme) { document.documentElement.dataset.docsRsTheme = theme; } } window.addEventListener("storage", ev => { if (ev.key === "rustdoc-theme") { applyTheme(ev.newValue); } }); // see ./storage-change-detection.html for details window.addEventListener("message", ev => { if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") { applyTheme(ev.data.storage.value); } }); applyTheme(window.localStorage.getItem("rustdoc-theme")); })();

 [Docs.rs](/)

{ "name": "vergen-git2", "version": "1.0.5" }

*   [vergen-git2-1.0.5](# "Generate 'cargo:rustc-env' instructions via 'build.rs' for use in your code via the 'env!' macro")

    *   vergen-git2 1.0.5
    *   [Permalink](/vergen-git2/1.0.5/vergen_git2/index.html "Get a link to this specific version")
    *   [Docs.rs crate page](/crate/vergen-git2/latest "See vergen-git2 in docs.rs")
    *   [MIT OR Apache-2.0](/crate/vergen-git2/latest)

    *   Links
    *   [Homepage](https://github.com/rustyhorde/vergen)
    *   [Repository](https://github.com/rustyhorde/vergen)
    *   [crates.io](https://crates.io/crates/vergen-git2 "See vergen-git2 in crates.io")
    *   [Source](/crate/vergen-git2/latest/source/ "Browse source of vergen-git2-1.0.5")

    *   Owners
    *   [CraZySacX](https://crates.io/users/CraZySacX)

    *   Dependencies
    *   *   [anyhow ^1.0.93 _normal_](/anyhow/^1.0.93)
        *   [derive\_builder ^0.20.2 _normal_](/derive_builder/^0.20.2)
        *   [git2 ^0.20.0 _normal_](/git2/^0.20.0)
        *   [time ^0.3.36 _normal_](/time/^0.3.36)
        *   [vergen ^9.0.4 _normal_](/vergen/^9.0.4)
        *   [vergen-lib ^0.1.6 _normal_](/vergen-lib/^0.1.6)
        *   [lazy\_static ^1.5.0 _dev_](/lazy_static/^1.5.0)
        *   [regex ^1.11.1 _dev_](/regex/^1.11.1)
        *   [serial\_test ^3.2.0 _dev_](/serial_test/^3.2.0)
        *   [temp-env ^0.3.6 _dev_](/temp-env/^0.3.6)
        *   [rustversion ^1.0.19 _build_](/rustversion/^1.0.19)




    *   Versions
    *

    *   [**100%** of the crate is documented](/crate/vergen-git2/latest)







*   [Platform](#)
    *   [i686-unknown-linux-gnu](/crate/vergen-git2/latest/target-redirect/i686-unknown-linux-gnu/vergen_git2/index.html)
    *   [x86\_64-unknown-linux-gnu](/crate/vergen-git2/latest/target-redirect/x86_64-unknown-linux-gnu/vergen_git2/index.html)
*   [Feature flags](/crate/vergen-git2/latest/features "Browse available feature flags of vergen-git2-1.0.5")

*   [Rust](#)
    *   [About docs.rs](/about)
    *   [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)
    *   [Rust website](https://www.rust-lang.org/)
    *   [The Book](https://doc.rust-lang.org/book/)
    *   [Standard Library API Reference](https://doc.rust-lang.org/std/)
    *   [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    *   [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
    *   [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)



## [vergen\_git2](../vergen_git2/index.html)1.0.5

*   [All Items](all.html)

### [Sections](#)

*   [vergen-git2 - Emit cargo instructions from a build script](#vergen-git2---emit-cargo-instructions-from-a-build-script "vergen-git2 - Emit cargo instructions from a build script")
    *   [Usage](#usage "Usage")
    *   [Features](#features "Features")
    *   [Environment Variables](#environment-variables "Environment Variables")

### [Crate Items](#structs)

*   [Structs](#structs "Structs")
*   [Enums](#enums "Enums")
*   [Traits](#traits "Traits")
*   [Type Aliases](#types "Type Aliases")

# Crate vergen\_git2Copy item path

[Source](../src/vergen_git2/lib.rs.html#9-483)

Expand description

## [§](#vergen-git2---emit-cargo-instructions-from-a-build-script)vergen-git2 - Emit cargo instructions from a build script

`vergen-git2` uses the [`git2`](https://docs.rs/git2) library to generate the git instructions.

`vergen-git2`, when used in conjunction with cargo [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script) can emit the following:

*   Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue) for each feature you have enabled. These can be referenced with the [`env`!](https://doc.rust-lang.org/nightly/core/macro.env.html "macro core::env") or [`option_env`!](https://doc.rust-lang.org/nightly/core/macro.option_env.html "macro core::option_env") macro in your code.
*   Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the [`fail_on_error`](struct.Emitter.html#method.fail_on_error "method vergen_git2::Emitter::fail_on_error") feature is not enabled and the requested variable is defaulted through error or the [`idempotent`](struct.Emitter.html#method.idempotent "method vergen_git2::Emitter::idempotent") flag.
*   Will emit [`cargo:rerun-if-changed=.git/HEAD`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) if git instructions are emitted. This is done to ensure any git instructions are regenerated when commits are made.
*   Will emit [`cargo:rerun-if-changed=.git/<path_to_ref>`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) if git instructions are emitted. This is done to ensure any git instructions are regenerated when commits are made.
*   Will emit [`cargo:rerun-if-changed=build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) to rerun instruction emission if the `build.rs` file changed.
*   Will emit [`cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
*   Will emit [`cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) to rerun instruction emission if the `SOURCE_DATE_EPOCH` environment variable has changed.

### [§](#usage)Usage

1.  Ensure you have build scripts enabled via the `build` configuration in your `Cargo.toml`

```toml
[package]
#..
build = "build.rs"
```

2.  Add `vergen-git2` as a build dependency in `Cargo.toml`, specifying the features you wish to enable.

```toml
[dependencies]
#..

[build-dependencies]
# All features enabled
vergen-git2 = { version = "1.0.0", features = ["build", "cargo", "rustc", "si"] }
# or
vergen-git2 = { version = "1.0.0", features = ["build"] }
# if you wish to disable certain features
```

3.  Create a `build.rs` file that uses `vergen-git2` to emit cargo instructions. Configuration starts with [`Emitter`](struct.Emitter.html "struct vergen_git2::Emitter"). Eventually you will call [`emit`](struct.Emitter.html#method.emit "method vergen_git2::Emitter::emit") to output the cargo instructions. See the [`emit`](struct.Emitter.html#method.emit "method vergen_git2::Emitter::emit") documentation for more robust examples.

##### [§](#generate-all-output)Generate all output

```
// NOTE: This will output everything, and requires all features enabled.
// NOTE: See the specific builder documentation for configuration options.
let build = BuildBuilder::all_build()?;
let cargo = CargoBuilder::all_cargo()?;
let git2 = Git2Builder::all_git()?;
let rustc = RustcBuilder::all_rustc()?;
let si = SysinfoBuilder::all_sysinfo()?;

Emitter::default()
    .add_instructions(&build)?
    .add_instructions(&cargo)?
    .add_instructions(&git2)?
    .add_instructions(&rustc)?
    .add_instructions(&si)?
    .emit()?;
```

##### [§](#sample-output)Sample Output

```text
cargo:rustc-env=VERGEN_BUILD_DATE=2024-01-31
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2024-01-31T03:26:34.065893658Z
cargo:rustc-env=VERGEN_CARGO_DEBUG=true
cargo:rustc-env=VERGEN_CARGO_FEATURES=
cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=0
cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=anyhow 1.0.79,vergen-pretty 0.3.2
cargo:rustc-env=VERGEN_GIT_BRANCH=master
cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=emitter@vergen.com
cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=Jason Ozias
cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=44
cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2024-01-30
cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=depsup
cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2024-01-30T21:43:43.000000000Z
cargo:rustc-env=VERGEN_GIT_DESCRIBE=0.1.0-beta.1-15-g728e25c
cargo:rustc-env=VERGEN_GIT_SHA=728e25ca5bb7edbbc505f12b28c66b2b27883cf1
cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2024-01-29
cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=5518eaa946291f00471af8b254b2a1715f234882
cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-unknown-linux-gnu
cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=17.0
cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.77.0-nightly
cargo:rustc-env=VERGEN_SYSINFO_NAME=Arch Linux
cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=Linux  Arch Linux
cargo:rustc-env=VERGEN_SYSINFO_USER=jozias
cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=31 GiB
cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=AuthenticAMD
cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=8
cargo:rustc-env=VERGEN_SYSINFO_CPU_NAME=cpu0,cpu1,cpu2,cpu3,cpu4,cpu5,cpu6,cpu7
cargo:rustc-env=VERGEN_SYSINFO_CPU_BRAND=AMD Ryzen Threadripper 1900X 8-Core Processor
cargo:rustc-env=VERGEN_SYSINFO_CPU_FREQUENCY=3792
cargo:rerun-if-changed=/home/jozias/projects/rust-lang/vergen-cl/.git/HEAD
cargo:rerun-if-changed=/home/jozias/projects/rust-lang/vergen-cl/.git/refs/heads/master
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
```

##### [§](#generate-specific-output)Generate specific output

```
// NOTE: This will output only the instructions specified.
// NOTE: See the specific builder documentation for configuration options.
let build = BuildBuilder::default().build_timestamp(true).build()?;
let cargo = CargoBuilder::default().opt_level(true).build()?;
let git2 = Git2Builder::default().commit_timestamp(true).build()?;
let rustc = RustcBuilder::default().semver(true).build()?;
let si = SysinfoBuilder::default().cpu_core_count(true).build()?;

Emitter::default()
    .add_instructions(&build)?
    .add_instructions(&cargo)?
    .add_instructions(&git2)?
    .add_instructions(&rustc)?
    .add_instructions(&si)?
    .emit()?;
```

##### [§](#sample-output-1)Sample Output

```text
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2024-01-31T03:26:34.065893658Z
cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=0
cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2024-01-30T21:43:43.000000000Z
cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.77.0-nightly
cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=8
cargo:rerun-if-changed=/home/jozias/projects/rust-lang/vergen-cl/.git/HEAD
cargo:rerun-if-changed=/home/jozias/projects/rust-lang/vergen-cl/.git/refs/heads/master
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
```

4.  Use the [`env!`](https://doc.rust-lang.org/nightly/core/macro.env.html "macro core::env") or [`option_env!`](https://doc.rust-lang.org/nightly/core/macro.option_env.html "macro core::option_env") macro in your code to read the environment variables.

```
if let Some(timestamp) = option_env!("VERGEN_BUILD_TIMESTAMP") {
    println!("Build Timestamp: {timestamp}");
}
if let Some(describe) = option_env!("VERGEN_GIT_DESCRIBE") {
    println!("git describe: {describe}");
}
```

### [§](#features)Features

`vergen-git2` has four main feature toggles allowing you to customize your output. No features are enabled by default.
You **must** specifically enable the features you wish to use.

FeatureEnables

build

`VERGEN_BUILD_*` instructions

cargo

`VERGEN_CARGO_*` instructions

rustc

`VERGEN_RUSTC_*` instructions

si

`VERGEN_SYSINFO_*` instructions

### [§](#environment-variables)Environment Variables

`vergen-git2` currently recognizes the following environment variables. The full list of the environment variable names can be found as [constants here](https://docs.rs/vergen-lib/latest/vergen_lib/constants/features/index.html)

VariableFunctionality

`VERGEN_IDEMPOTENT`

If this environment variable is set `vergen` will use the idempotent output feature regardless of the configuration set in `build.rs`. This exists mainly to allow package maintainers to force idempotent output to generate deterministic binary output.

`SOURCE_DATE_EPOCH`

If this environment variable is set `vergen` will use the value (unix time since epoch) as the basis for a time based instructions. This can help emit deterministic instructions.

`VERGEN_BUILD_*`

If this environment variable is set `vergen` will use the value you specify for the output rather than generating it.

`VERGEN_CARGO_*`

If this environment variable is set `vergen` will use the value you specify for the output rather than generating it.

`VERGEN_GIT_*`

If this environment variable is set `vergen` will use the value you specify for the output rather than generating it.

`VERGEN_RUSTC_*`

If this environment variable is set `vergen` will use the value you specify for the output rather than generating it.

`VERGEN_SYSINFO_*`

If this environment variable is set `vergen` will use the value you specify for the output rather than generating it.

## Structs[§](#structs)

*   [BuildBuilder](struct.BuildBuilder.html "struct vergen_git2::BuildBuilder")`build`

    Builder for [`Build`](struct.Build.html).

*   [CargoBuilder](struct.CargoBuilder.html "struct vergen_git2::CargoBuilder")`cargo`

    Builder for [`Cargo`](struct.Cargo.html).

*   [CpuRefreshKind](struct.CpuRefreshKind.html "struct vergen_git2::CpuRefreshKind")`si`

    Used to determine what you want to refresh specifically on the [`Cpu`](https://docs.rs/sysinfo/0.33.1/x86_64-unknown-linux-gnu/sysinfo/common/system/struct.Cpu.html "struct sysinfo::common::system::Cpu") type.

*   [DefaultConfig](struct.DefaultConfig.html "struct vergen_git2::DefaultConfig")

    The default configuration to use when an issue has occured generating instructions

*   [Emitter](struct.Emitter.html "struct vergen_git2::Emitter")

    The `Emitter` will emit cargo instructions (i.e. cargo:rustc-env=NAME=VALUE) base on the configuration you enable.

*   [Git2](struct.Git2.html "struct vergen_git2::Git2")

    The `VERGEN_GIT_*` configuration features

*   [Git2Builder](struct.Git2Builder.html "struct vergen_git2::Git2Builder")

    Builder for [`Git2`](struct.Git2.html).

*   [MemoryRefreshKind](struct.MemoryRefreshKind.html "struct vergen_git2::MemoryRefreshKind")`si`

    Used to determine which memory you want to refresh specifically.

*   [ProcessRefreshKind](struct.ProcessRefreshKind.html "struct vergen_git2::ProcessRefreshKind")`si`

    Used to determine what you want to refresh specifically on the [`Process`](https://docs.rs/sysinfo/0.33.1/x86_64-unknown-linux-gnu/sysinfo/common/system/struct.Process.html "struct sysinfo::common::system::Process") type.

*   [RefreshKind](struct.RefreshKind.html "struct vergen_git2::RefreshKind")`si`

    Used to determine what you want to refresh specifically on the [`System`](https://docs.rs/sysinfo/0.33.1/x86_64-unknown-linux-gnu/sysinfo/common/system/struct.System.html "struct sysinfo::common::system::System") type.

*   [RustcBuilder](struct.RustcBuilder.html "struct vergen_git2::RustcBuilder")`rustc`

    Builder for [`Rustc`](struct.Rustc.html).

*   [SysinfoBuilder](struct.SysinfoBuilder.html "struct vergen_git2::SysinfoBuilder")`si`

    Builder for [`Sysinfo`](struct.Sysinfo.html).


## Enums[§](#enums)

*   [DependencyKind](enum.DependencyKind.html "enum vergen_git2::DependencyKind")`cargo`

    Dependencies can come in three kinds


## Traits[§](#traits)

*   [AddCustomEntries](trait.AddCustomEntries.html "trait vergen_git2::AddCustomEntries")

    This trait should be implemented to allow the `vergen` emitter to properly emit your custom instructions.


## Type Aliases[§](#types)

*   [CargoRerunIfChanged](type.CargoRerunIfChanged.html "type vergen_git2::CargoRerunIfChanged")

    The vector of strings used to emit `cargo:rerun-if-changed=VALUE` cargo instructions

*   [CargoWarning](type.CargoWarning.html "type vergen_git2::CargoWarning")

    The vector of strings used to emit `cargo:warning=VALUE` cargo instructions


{"&<Vec<T, A> as Index<I>>::Output":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","&mut <Vec<T, A> as Index<I>>::Output":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","<Vec<T, A> as IntoIterator>::IntoIter":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","<Vec<T> as IntoDeserializer<'de, E>>::Deserializer":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","<Vec<T> as IntoParallelIterator>::Iter":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\\" title=\\"struct alloc::vec::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>"}
