# Benchmark

## TCP-HTTP server benchmark

Using Apache Benchmark (AB).

### Static response

``` bash
process 1: $ cargo run --example static localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://localhost:8888/
```

*Expected mean:* 4ms

### Simple dynamic response

``` bash
process 1: $ cargo run --example dynamic localhost 8888 10 index.htm ./html/ 404.htm 1024
process 2: $ ab -n 10000 -c 10 http://localhost:8888/?test=abcdef
```

*Expected mean:* 2ms



[Back to start](https://github.com/cjohansson/milstian-rust-internet-framework/)
