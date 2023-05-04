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

For the tests, do:

```bash
~/rust-auth/backend$ cargo test
```

If you get an error that looks like this, the `"Too many open files"` error:

```shell
---- users::register::test_register_user_failure_email stdout ----
thread 'actix-server worker 7' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 24, kind: Uncategorized, message: "Too many open files" }', /Users/joidogun/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-server-2.2.0/src/worker.rs:400:34
```

It has nothing to do with the tests failing. You can find explanation and fix [here][2] or temporarily increase the system-wide maximum number of file handles (I used `50000` here):

```shell
ulimit -n 50000
```

[1]: https://crates.io/crates/cargo-watch "Cargo watch"
[2]: https://www.phind.com/search?cache=9d09941e-bee6-4892-a3a9-7f0c76d5aea9 "Too many open files error"
