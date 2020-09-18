# loadem

Command line tool for emulating web load from thousands of clients.

Built with async [Rust](https://www.rust-lang.org/) using [Tokio](https://tokio.rs/),
[hyper](https://hyper.rs/) and [rustls](https://docs.rs/rustls).

This is the spiritual successor to [OpenWebLoad](https://pjoe.github.io/openwebload/)

## Installation

Download binaries from https://github.com/pjoe/loadem/releases and add to you path.

### Windows

You can also install with [Chocolatey](https://chocolatey.org/)

In PowerShell with Admin rights:

```
choco install loadem
```

## Usage

```
$ loadem http://localhost 200
URL: http://localhost/
Clients: 200
Starting
MaTps 3410.85, Tps 3410.85, Err  0.00%, Resp Time  0.026
MaTps 3718.58, Tps 4026.32, Err  0.00%, Resp Time  0.022
MaTps 3732.09, Tps 3759.11, Err  0.00%, Resp Time  0.022
MaTps 3871.03, Tps 4287.86, Err  0.00%, Resp Time  0.020
MaTps 3941.79, Tps 4224.79, Err  0.00%, Resp Time  0.020
MaTps 4027.94, Tps 4458.70, Err  0.00%, Resp Time  0.019
MaTps 4102.18, Tps 4547.63, Err  0.00%, Resp Time  0.017
^C
Completed 30743 requests in 7.47 seconds
Total TPS: 4114.94
Avg. Response time:  0.021
Max Response time:   3.155
```
