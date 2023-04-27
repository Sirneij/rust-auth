# Backend

## Run locally

Once you've cloned the application, you can either run the app using Docker, a Dockerfile is available, or just use cargo:

```bash
cargo run
```

If you want `cargo` to restart after every change you make to the application, you need to install [cargo-watch][1]:

```bash
~/rust-auth/backend$ cargo add cargo-watch
```

And then:

```bash
~/rust-auth/backend$ cargo watch -x 'run -- --release'
```

[1]: https://crates.io/crates/cargo-watch "Cargo watch"
