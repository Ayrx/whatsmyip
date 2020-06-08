# whatsmyip

Small web service to answer the "what's my ip address" query.

```
➜ curl http://127.0.0.1:1337/whatsmyip
100.100.100.100⏎
```

## Building

`whatsmyip` can be compiled to a Linux static binary using MUSL. Add the
appropriate [target][platform-support] (e.g. `x86_64-unknown-linux-musl`) and
build with the `--target` flag.

```
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --target x86_64-unknown-linux-musl --release
```

[platform-support]: https://forge.rust-lang.org/release/platform-support.html
