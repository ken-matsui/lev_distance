# lev_distance

A copy of Levenshtein distance implementation from [Rust Compiler](https://github.com/rust-lang/rust/blob/0fb1c371d4a14f9ce7a721d8aea683a6e6774f6c/compiler/rustc_span/src/lev_distance.rs).

This package uses `String` instead of [`Symbol`](https://github.com/rust-lang/rust/blob/d6082292a6f3207cbdacd6633a5b9d1476bb6772/compiler/rustc_span/src/symbol.rs#L1625) of the Rust Compiler for general usage.

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

## [LICENSE](https://github.com/ken-matsui/lev_distance/blob/main/src/lib.rs#L1-L26)

This package is released under the [MIT license](https://github.com/rust-lang/rust/blob/master/COPYRIGHT) from [Rust Compiler](https://github.com/rust-lang/rust/blob/0fb1c371d4a14f9ce7a721d8aea683a6e6774f6c/compiler/rustc_span/src/lev_distance.rs).

### Rust Compiler

```
Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```

The license for this package is placed [here](LICENSE).
