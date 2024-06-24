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


## Support

This is an open source project, so there isn't support per se. If you open an issue in the
repository, we'll try and help you, but no promises.

---

<div align="center">
© Copyright 2024 <b>bmc::labs</b> GmbH. All rights reserved.<br />
<em>solid engineering. sustainable code.</em>
</div>
