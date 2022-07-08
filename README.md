# Macros for target-lexicon

Provides a `triple` macro to construct static a [target-lexicon Triple](https://docs.rs/target-lexicon/latest/target_lexicon/struct.Triple.html).

## Example

```rust
use target_lexicon::Triple;
use target_lexicon_macros::triple;

const TARGET: Triple = triple!("x86_64-unknown-linux-gnu")

fn main() {
    dbg!(TARGET);
}
```