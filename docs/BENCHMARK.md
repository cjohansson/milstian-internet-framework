# Benchmark

This document will contain information about how to do benchmarks and how their score is.

## TCP-HTTP server benchmark

Using Apache Benchmark (AB).

### Static response direct to port

``` bash
process 1: $ cargo run --example static localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://localhost:8888/
```

*Expected total mean:* 2ms

### Static response via NGINX virtual host proxy

This requires a virtual host proxy setup as described [here](NGINX.md).

``` bash
process 1: $ cargo run --example static localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 5000 -c 10 http://milstian.test/
```

*Expected total mean:* 3ms


### Simple dynamic response direct to port

``` bash
process 1: $ cargo run --example dynamic localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://localhost:8888/?test=abcdef
```

*Expected total mean:* 2ms

### Simple dynamic response via NGINX virtual host proxy

This requires a virtual host proxy setup as described [here](NGINX.md).

``` bash
process 1: $ cargo run --example dynamic localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 5000 -c 10 http://milstian.test/?test=abcdef
```

*Expected total mean:* 3ms

**NOTE** Experiencing some kind of limit in the NGINX proxy once requests pass over 5000, but it seems to be unrelated to milstian.



[Back to start](../../../)
