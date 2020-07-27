Minisynth-rs Clone
------------------

A rust implementation of [minisynth](https://www.cs.cornell.edu/~asampson/blog/minisynth.html), following Nick Fitzgerald's post [Program Synthesis is Possible in Rust](https://fitzgeraldnick.com/2018/11/15/program-synthesis-is-possible-in-rust.html). Indeed, you will find that it is nearly identical to his [minisynth-rs](https://github.com/fitzgen/minisynth-rs), with the following changes:

1. I used [simple-error](https://crates.io/crates/simple-error) instead of [failure](https://crates.io/crates/failure), since the latter is deprecated.
2. I made it a binary program rather than a library.
3. The [Z3](https://crates.io/crates/z3) crate from crates.io is used instead of [z3-rs](https://github.com/fitzgen/z3-rs)
4. There are no tests.

Usage
-----

```
$ cargo run --release <spec> <template>
```

For example:

```
$ cargo run -q --release "x * 2" "(x << h1)"
{"h1": 1}
$ cargo run -q --release "x * 10" "(x << h1) + (x << h2)"
{"h1": 1, "h2": 3}
$ cargo run -q  "x * 9" "x << (hb1 ? x : hn1) + (hb2 ? x : hn2)"
{"hn1": 3, "hb1": 0, "hb2": -1, "hn2": 0}
```
