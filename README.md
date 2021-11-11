# lev_distance

A copy of Levenshtein distance implementation from Rust Compiler.

The Rust Compiler has [`Symbol`](https://github.com/rust-lang/rust/blob/d6082292a6f3207cbdacd6633a5b9d1476bb6772/compiler/rustc_span/src/symbol.rs#L1625) for internal usage.
However, it is not necessary in general and replaced with `String`.

## Example

```rust
use lev_distance::find_best_match_for_name;

fn main() {
    let v = vec!["aaa", "bbb"];
    let lookup = "aa";
    
    if let Some(sugg) = match find_best_match_for_name(v.iter(), "aa", None) {
        Some(sugg) if sugg == lookup => None,
        sugg => sugg,
    } {
        println!("Did you mean `{}`?", sugg);
    }
}
```

```shell
$ cargo run
Did you mean `aaa`?
```
