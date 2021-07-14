For reproducing possible issues with xh/reqwest.

[Install Rust](https://www.rust-lang.org/tools/install), clone the repository, and inside it, run `RUST_LOG=trace cargo run <url>`. You should see something like this when it's done compiling:

```
$ RUST_LOG=trace cargo run http://example.org
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/simreq 'http://example.org'`
 TRACE reqwest::blocking::wait > (ThreadId(1)) park without timeout
 TRACE reqwest::blocking::client > (ThreadId(2)) start runtime::block_on
 TRACE reqwest::blocking::wait   > wait at most 30s
 TRACE reqwest::blocking::wait   > (ThreadId(1)) park timeout 29.999992606s
 DEBUG reqwest::connect          > starting new connection: http://example.org/
 TRACE mio::poll                 > registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
 TRACE want                      > signal: Want
 TRACE want                      > signal found waiting giver, notifying
 TRACE want                      > poll_want: taker wants!
 TRACE want                      > signal: Want
 TRACE want                      > signal: Want
 DEBUG reqwest::async_impl::client > response '200 OK' for http://example.org/
Response {
    url: Url {
        scheme: "http",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "example.org",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    status: 200,
    headers: {
        "age": "184384",
        "cache-control": "max-age=604800",
        "content-type": "text/html; charset=UTF-8",
        "date": "Wed, 14 Jul 2021 20:16:49 GMT",
        "etag": "\"3147526947+ident\"",
        "expires": "Wed, 21 Jul 2021 20:16:49 GMT",
        "last-modified": "Thu, 17 Oct 2019 07:18:26 GMT",
        "server": "ECS (dcb/7EEE)",
        "vary": "Accept-Encoding",
        "x-cache": "HIT",
        "content-length": "1256",
    },
}
 TRACE want                        > signal: Want
 TRACE reqwest::blocking::client   > closing runtime thread (ThreadId(2))
 TRACE reqwest::blocking::client   > signaled close for runtime thread (ThreadId(2))
 TRACE reqwest::blocking::client   > (ThreadId(2)) Receiver is shutdown
 TRACE reqwest::blocking::client   > (ThreadId(2)) end runtime::block_on
 TRACE mio::poll                   > deregistering event source from poller
 TRACE want                        > signal: Closed
 TRACE reqwest::blocking::client   > (ThreadId(2)) finished
 TRACE reqwest::blocking::client   > closed runtime thread (ThreadId(2))
```
