<div align="center">

<img src="./assets/runrs-banner-1024px.jpg" />
<br/>

# `🏃🏽 runrs`

**Manage GitLab Runners in Docker via REST**

</div>

If you're running GitLab runners in Docker and you're looking for a simple, non-manual way of doing so, runrs has you covered. You can run runrs in your Docker alongside the GitLab Runners control service - it just needs access to the configuration file containing Runner configs and things will _just work™_.


## Quickstart

_some instructions for how to run it locally for testing_


## Motivation

Running GitLab Runners in Docker is simple, but managing them - adding new ones, cleaning up, rotating them - is an annoying manual task. If you're anything like us, you don't like those. Moreover, it's something we don't do _that often_ - a few times per month perhaps - so we end up looking up the manual steps in the GitLab docs every godforsaken time. Meanwhile, pretty much everything else we run is Terraformed, which makes this doubl-y annoying.

Thus, we wrote this simple service which provides a CRUD API for GitLab Runner configurations and can run in Docker, managing the Runners configuration file. We also have [a Terraform provider]() you can use to _GitLab Runner Setup as Code_ your Runners.


## Using runrs

_instructions on how to get set up for real, and a link to an actual example_


## Local Development Setup

It's a fairly Rust and `cargo` project, so if you have that up and running, you should be good. For testing, also a Docker would be great. But hey, I mean, you do you.

### Prerequisites

- well, git
- a recent (1.75+) Rust toolchain

### Setup

1. Clone the thing.
   ```bash
   git clone git@github.com:bmc-labs/runrs.git
   cd runrs
   ```
1. Build and run the thing.
   ```bash
   cargo run
   ```

Similarly, testing is via `cargo test`, as you might have expected.

That's it. Make a PR with your changes and we'll talk about them.


## Support

This is an open source project, so there isn't support per se. If you open an issue in the repository, we'll try and help you, but no promises.

---

<div align="center">
© Copyright 2024 <b>bmc::labs</b> GmbH. All rights reserved.<br />
<em>solid engineering. sustainable code.</em>
</div>
