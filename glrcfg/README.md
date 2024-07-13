<div align="center">

# `🏃🏽 glrcfg`

**Rust Implementation of the GitLab Runners Advanced Configuration File Format**

</div>

If, for some unfortunate whim of the universe, you find yourself needing to generate the
configuration file required for advanced GitLab runner setup - the documentation of which you can
retrieve [here](https://docs.gitlab.com/runner/configuration/advanced-configuration.html) - and you
furthermore need to do so using Rust, this is the library for you.


## Motivation

For us, the answer is simple: [runrs](https://github.com/bmc-labs/runrs). Also, our primary language
is Rust and since nothing like this existed in Python either, we decided it would be a fun
contribution to the Rust ecosystem. Even though it's a bit niche.

[(We do resort to Go sometimes](https://github.com/bmc-labs/terraform-provider-peripheral), but we'd
like to keep it to a minimum.)


## Usage

Run `cargo add glrcfg` and you're good to go. There's docs and there's [the official GitLab docs for
the format](https://docs.gitlab.com/runner/configuration/advanced-configuration.html) - we keep all
terminology and defaults exactly as they are there.

Take a look at the `glrcfg` crate's documentation for details on how to use it, specifically its
feature flags. There is a `tracing` feature which turns on some logging via `tracing`, and an `sqlx`
feature which implements the [SQLx traits](https://docs.rs/sqlx/latest/sqlx/#traits) `sqlx::Type`,
`sqlx::Encode` and `sqlx::Decode` traits for our types so you use them as database fields.

### A word on ergonomics

You'll find that all components of the configuration file are implemented as structs which have all
their fields as `pub` and which implement `Default::default`. This way, you can simply create the
components from whatever data model you have using a `Component { field: value, ..Default::default()
}` pattern.

The components have certain semantics. As an example, the `concurrent` field in the global section
must be a non-zero positive integer, the `log_level` field must be one of a list of log levels, and
the `connection_max_age` must be a Golang duration string (e.g. `1h30m`). This library uses or
implements types which enforce these constraints, so that invalid configurations are impossible to
create. In other words: it is not possible to just pass a `&str` as a Golang duration string - you
must use `GolangDuration::parse("1h30m")` (or `"1h30m".parse()`) and pass the result. 


## Support

This is an open source project, so there isn't support per se. If you open an issue in the
repository, we'll try and help you, but no promises.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

---

<div align="center">
© Copyright 2024 <b>bmc::labs</b> GmbH. All rights reserved.<br />
<em>solid engineering. sustainable code.</em>
</div>
