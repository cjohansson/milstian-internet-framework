# Benchmark

This document will contain information about how to do benchmarks and how their score is.

## TCP-HTTP server benchmark

Using Apache Benchmark (AB).

### Static response direct to port

``` bash
process 1: $ cargo run --example static localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://localhost:8888/
```

*Expected mean:* 4ms

### Static response via NGINX virtual host proxy

This requires a virtual host proxy setup as described [here](NGINX.md).

``` bash
process 1: $ cargo run --example static localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://milstian.test/
```

*Expected mean:* -


### Simple dynamic response direct to port

``` bash
process 1: $ cargo run --example dynamic localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://localhost:8888/?test=abcdef
```

*Expected mean:* 2ms

### Simple dynamic response via NGINX virtual host proxy

This requires a virtual host proxy setup as described [here](NGINX.md).

``` bash
process 1: $ cargo run --example dynamic localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://milstian.test/?test=abcdef
```

*Expected mean:* -



[Back to start](../../../)
