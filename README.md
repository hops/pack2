# pack2

pack2 is meant to be a replacement for iphelix's [PACK](https://github.com/iphelix).
This is a work in progress. Not all features are available and while being
similiar some will differ slightly.

## note

It's in a totally unfinished state but I wanted to have it public before [CMIYC 2020](https://contest-2020.korelogic.com)

# requirements

You need to have Rust installed. The recommended way to do so is using [rustup](https://rustup.rs).

# building

```
$ cargo build --release
```
Your binary will be located at `target/release/pack2`  
To squeeze out more optimizations we use some compiler flags in the release profile.  
This results in a longer compile time in favor of running a bit faster.

# design principles
Unless stated otherwise all tools adhere the following rules:

- If no input file is specified it reads from `stdin`
- If no output file is specified it writes to `stdout`
- Infos (e.g. stats) are always written to `stderr`
- Input lines in the $HEX[] format are always decoded before any further processing
- If at least one character of an output line is outside of \x20 - \x7e it will be encoded in the $HEX[] format

# usage
## filtermask
__Note:__ This will fail horribly in many cases. See [#6](https://github.com/hops/pack2/issues/6) for more details.  
Filters the input by a given mask, only writing the lines that match the mask.
```
$ cat input.txt
test
foobar
Password

$ pack2 filtermask ?l?l?l?l input.txt
test
```

## rulegen
There were plans to integrate this as well but since [rulesfinder](https://github.com/synacktiv/rulesfinder)
(also written in Rust) got released there's really no point reinventing the wheel.

## statsgen
Generates statistics of a given wordlist.

```
$ pack2 statsgen < input.txt
[+] Analyzed 3 / 3 passwords.
[*] Length distribution: (min: 4 max: 8)
[+]                          4: 33.33% (1)
[+]                          6: 33.33% (1)
[+]                          8: 33.33% (1)

[*] Charset distribution:                    count   min   max
[+]                 loweralpha: 66.67%           2     4     6
[+]                 mixedalpha: 33.33%           1     8     8

[*] Simple masks distribution:               count   min   max
[+]                     string: 100.00%          3     4     8

[*] Masks (top 25):
[+]                   ?l?l?l?l: 33.33% (1)
[+]           ?u?l?l?l?l?l?l?l: 33.33% (1)
[+]               ?l?l?l?l?l?l: 33.33% (1)
?l?l?l?l	33.3333	1
?u?l?l?l?l?l?l?l	33.3333	1
?l?l?l?l?l?l	33.3333	1
```

You can also provide the `-o` flag to specify the output file.
Type `pack2 help statsgen` to see all options.

## unhex
Decodes and writes lines using the $HEX[] format. Lines not using said format
are unaffected and written as is.  
```
$ echo '$HEX[52c3b67363687469]' | pack2 unhex
Röschti
```
