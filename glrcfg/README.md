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

Well, `cargo add glrcfg` and you're good to go. There's docs and there's [the official GitLab docs
for the format](https://docs.gitlab.com/runner/configuration/advanced-configuration.html) - we keep
all terminology and defaults exactly as they are there.

### A word on ergonomics

You'll find that all components of the configuration file are implemented as structs which have all
their fields as `pub` and which implement `Default::default`. This way, you can simply create the
components from whatever data model you have using a `Component { field: value, ..Default::default()
}` pattern.

The components have certain semantics. As an example, the `concurrent` field in the global section
must be a non-zero positive integer, the `log_level` field must be one of a list of log leves, and
the `connection_max_age` must be a Golang duration string (e.g. `1h30m`).

Some of these constraints can only be checked at run time.  In other words, `concurrent` is a `u32`
and there's an enum for `log_level` - but you can still set `concurrent` to zero and pass
`connection_max_age` any random string. We could, of course, have solved this via newtypes which
implement the constraints, but that wouldn't have looked very good with respect to the above
described ergonomics, so we didn't.

The components do, of course, have certain semantics which are not enforced this way
(although, for example, if something expects a URL, you'll have to hand it a `url::Url` type, and
such). As an example, if you set `concurrent` in the global section to zero, the resulting config
will be invalid and `gitlab-runner` will not accept it.




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
