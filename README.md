# loadem

Command line tool for emulating web load from thousands of clients.

Built with async [Rust](https://www.rust-lang.org/) using [Tokio](https://tokio.rs/),
[hyper](https://hyper.rs/) and [rustls](https://docs.rs/rustls).

This is the spiritual successor to [OpenWebLoad](https://pjoe.github.io/openwebload/)

## Installation

Download binaries from https://github.com/pjoe/loadem/releases and add to you path.

## Usage

```
$ loadem http://localhost 200
URL: http://localhost/
Clients: 200
Starting
Tps 2778.67 Err  0.00%
Tps 3674.29 Err  0.00%
Tps 3565.66 Err  0.00%
Tps 4097.57 Err  0.00%
Tps 3796.36 Err  0.00%
^C
Done
```
