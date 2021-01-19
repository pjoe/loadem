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
URL: http://localhost
Clients: 200
Starting
MaTps 4777.17, Tps 4777.17, Err  0.00%, Lat Avg  0.018, P50  0.009, P99  0.200, P99.9  0.220
MaTps 4912.00, Tps 5046.83, Err  0.00%, Lat Avg  0.016, P50  0.009, P99  0.157, P99.9  0.341
MaTps 4929.57, Tps 4964.71, Err  0.00%, Lat Avg  0.014, P50  0.009, P99  0.111, P99.9  0.191
MaTps 4470.94, Tps 3095.06, Err  0.00%, Lat Avg  0.020, P50  0.015, P99  0.152, P99.9  0.238
MaTps 4356.89, Tps 3900.69, Err  0.00%, Lat Avg  0.023, P50  0.011, P99  0.429, P99.9  0.469
MaTps 4302.34, Tps 4029.58, Err  0.00%, Lat Avg  0.021, P50  0.012, P99  0.158, P99.9  0.470
MaTps 4295.27, Tps 4252.88, Err  0.00%, Lat Avg  0.022, P50  0.010, P99  0.540, P99.9  0.543
^C
URL: http://localhost
Clients: 200
Completed 31825 requests in 7.45 seconds
Total TPS: 4274.48
Latency:
 Avg.   0.019
 P50    0.010
 P99    0.200
 P99.9  0.541
 Max    0.544
```
