<div align="center">

<img src="./assets/runrs-banner-1024px.jpg" />
<br/>

# `🏃🏽 runrs`

**Manage GitLab Runners in Docker via REST**

</div>

If you're running GitLab runners in Docker and you're looking for a simple, non-manual way of doing
so, runrs has you covered. You can run runrs in your Docker alongside the GitLab Runners control
service - it just needs access to the configuration file containing Runner configs and things will
_just work™_.


## Motivation

Running GitLab Runners in Docker is simple, but managing them - adding new ones, cleaning up,
rotating them - is an annoying manual task. If you're anything like us, you don't like those.
Moreover, it's something we don't do _that often_ - a few times per month perhaps - so we end up
looking up the manual steps in the GitLab docs every godforsaken time. Meanwhile, pretty much
everything else we run is Terraformed, which makes this doubl-y annoying.

Thus, we wrote this simple service which provides a CRUD API for GitLab Runner configurations and
can run in Docker, managing the Runners configuration file. We also have [a Terraform
provider](https://github.com/bmc-labs/terraform-provider-peripheral) you can use to _GitLab Runner
Setup as Code_ your Runners.


## Using runrs in prod

We don't recommend you do (just yet). If you insist: use the Docker container we provide, and make
sure it has access to the GitLab Runner configuration file. You do so by passing the path to it via
the `CONFIG_PATH` environment variable.

If you want to persist the SQLite database (e.g. because you want your runner setup to survive
reboots, or because you're running several replicas of `runrs` for some reason), you can pass it any
URL that SQLite would understand - most commonly a path to a file on disk - via the `DATABASE_URL`
environment variable.


## Local Development Setup

It's a vanilla Rust and `cargo` project, so if you have a recent (1.75+) Rust toolchain installed,
you should be good. For testing end to end, you'll need:

- Docker, which is going to be used as the executor for your runners, up and running, and
- [Docker Compose](https://docs.docker.com/compose/install/)

It's _also_ a nixified project using a flake, so if you prefer to use that to build it, you can.
Look at the `flake.nix` for current package targets. In that vein: there is also a
[`direnv`](https://direnv.net/) setup in the project, so if you have that installed, you can just
`direnv allow` in the project root and you'll have the setup done.

### Setup

1. Clone the thing.
   ```bash
   git clone git@github.com:bmc-labs/runrs.git
   cd runrs
   ```
1. Use Docker Compose to run the `gitlab-runner` service with correct setup in Docker.
   ```bash
   docker compose up -d
   ```
1. Build and run the thing.
   ```bash
   cargo run
   ```

Similarly, testing is via `cargo test`, as you might have expected.

If you are building with nix, you can use the `nix` command to build the project:

```bash
# to build the default target, which is the "runrs" binary:
nix build

# to build the Docker image:
nix build .#runrs-docker-image

# after building the Docker image, you need to load it:
docker load < result
```

That's it. Make a PR with your changes and we'll talk about them.


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
