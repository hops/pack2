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
To squeeze out more optimizations we use some flags in the release profile.  
This results in a longer compile time in favor of running a bit faster.

# usage

As of now (version 0.1.0) only statsgen is implemented. It reads a wordlist
from `stdin`. By default masks are written to `stdout` while the stats are written to
`stderr`.

```
$ ./target/release/pack2 statsgen < input.txt > masks.txt
```

You can also provide the `-o` flag to specify the output file.
Type `pack2 help statsgen` to see all options.
