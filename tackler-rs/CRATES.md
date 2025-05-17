# Tackler-NG: Rusty Services

[Tackler](https://tackler.fi/) is fast, reliable bookkeeping tool 
with native GIT SCM support for plain text accounting, written in Rust.

These are Rusty Services for [Tackler CLI](https://crates.io/crates/tackler).

The Rusty Services are assorted bits and pieces which are needed for 
Tackler, but won't fit into the domain of plain text accounting.


## Full haystack regex matchers

By default Rust `regex::Regex::is_match` will test if there is a match for the regex [anywhere in the haystack](https://docs.rs/regex/latest/regex/struct.Regex.html#method.is_match) given.

These constructors create a regex which will try to match against the full haystack by default. This logic is similar than [java.util.regex.Matcher.matches()](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/util/regex/Matcher.html#matches())

```rust
tackler_rs::regex::{
    new_full_haystack_regex,
    new_full_haystack_regex_set,
    peeled_pattern,
    peeled_patterns
}
```

### Serializers and Deserializers for full haystack matchers

This is serializer and deserializer implementation of full haystack matcher for Serde.

```rust
tackler_rs::regex::serde::full_haystack_matcher
```


## Tackler components on Crates.io

* Tackler CLI application: [tackler](https://crates.io/crates/tackler)
* Tackler Client API: [tackler-api](https://crates.io/crates/tackler-api)
* Tackler Server API: [tackler-core](https://crates.io/crates/tackler-core)
* Tackler Rusty Services: [tackler-rs](https://crates.io/crates/tackler-rs)
