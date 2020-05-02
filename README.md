# pack2

pack2 is ment to be a replacement for iphelix's [PACK](https://github.com/iphelix).
This is a work in progress. Not all features are available and while being
similiar some features will differ slightly.


# requirements

You need Rust installed. The easiest way to do so is using [rustup](https://rustup.rs).

# building

```
$ cargo build --release
```
Your binary will be located at `target/release/pack2`  
To squeeze out more compiler optimizations use the following command. This
will result in a longer compile time in favor of running a bit faster.
```
$ RUSTFLAGS="-C opt-level=3 -C target-cpu=native -C codegen-units=1" cargo build --release
```

# usage

As of now (version 0.1.0) only statsgen is implemented. It reads a wordlist
from `stdin`. Masks are written to `stdout` while the stats are written to
`stderr`.

```
$ ./target/release/pack2 < input.txt > masks.txt
```

